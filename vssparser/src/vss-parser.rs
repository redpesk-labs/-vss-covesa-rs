/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 */

use crate::types::*;
use crate::units::*;
use crate::utils::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while_m_n},
    character::complete::{alphanumeric1, char, newline, not_line_ending, space0, space1},
    combinator::{eof, opt},
    error::{Error, ErrorKind},
    sequence::tuple,
    IResult,
};

// error ref: https://github.com/rust-bakery/nom/blob/main/doc/error_management.md
fn nom_to_code_error<'a>(error: nom::Err<Error<&'a str>>) -> (&'a str, ErrorKind) {
    match error {
        nom::Err::Error(error) => (error.input, error.code),
        nom::Err::Incomplete(_error) => ("label-not-found", ErrorKind::Fail),
        nom::Err::Failure(error) => (error.input, error.code),
    }
}

type IndentCallback = fn(input: &str, indent: usize) -> IResult<&str, VssElement>;

fn open_bracket(s: &str) -> IResult<&str, char> {
    char('[')(s)
}

fn close_bracket(s: &str) -> IResult<&str, char> {
    char(']')(s)
}

fn quote<'a>(input: &'a str) -> IResult<&'a str, char> {
    let (input, char) = alt((char('"'), char('\'')))(input)?;
    Ok((input, char))
}

fn sharp(s: &str) -> IResult<&str, char> {
    char('#')(s)
}

fn dash(s: &str) -> IResult<&str, char> {
    char('-')(s)
}

fn colum(s: &str) -> IResult<&str, char> {
    char(':')(s)
}

fn comma(s: &str) -> IResult<&str, char> {
    char(',')(s)
}

fn _ignore_empty_lines(mut input: &str) -> &str {
    loop {
        match empty_line(input) {
            Ok((pointer, _)) => input = pointer,
            Err(_) => break input,
        }
    }
}

fn end_of_file<'a>(input: &'a str) -> IResult<&'a str, char> {
    let _ = eof(input)?;
    Ok((input, ' '))
}

// search \n or eof
fn eol<'a>(input: &'a str) -> IResult<&'a str, char> {
    let (input, char) = alt((newline, end_of_file))(input)?;
    Ok((input, char))
}

fn is_valid_string1(chr: char) -> bool {
    chr != '"' && chr.is_ascii()
}
fn is_valid_string2(chr: char) -> bool {
    chr != '\'' && chr.is_ascii()
}

fn vss_string<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, _) = space0(input)?;
    let (input, quote) = quote(input)?;
    let (input, text) = if quote == '"' {
        let (input, text) = take_while(is_valid_string1)(input)?;
        let (input, _) = char('"')(input)?;
        (input, text)
    } else {
        let (input, text) = take_while(is_valid_string2)(input)?;
        let (input, _) = char('\'')(input)?;
        (input, text)
    };
    let (input, _) = space0(input)?;
    if text.len() == 0 {
        return Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Eof,
        }));
    }
    Ok((input, text.to_owned()))
}

fn string_or_number<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, value) = alt((vss_number, vss_string))(input)?;
    Ok((input, value))
}

// vss vss_path accept everything but space
fn is_valid_pathname(chr: char) -> bool {
    !chr.is_ascii_whitespace() && chr.is_ascii()
}

fn vss_path<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, text) = take_while(is_valid_pathname)(input)?;
    Ok((input, text.to_owned()))
}

// argument return a single word
fn is_valid_argument(chr: char) -> bool {
    chr.is_alphanumeric() || chr == '.'
}

fn argument<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, text) = take_while(is_valid_argument)(input)?;
    Ok((input, text.to_owned()))
}

fn is_valid_numeric(chr: char) -> bool {
    chr.is_numeric() || chr == '-' || chr == '.'
}
fn vss_number<'a>(input: &'a str) -> IResult<&'a str, String> {
    let (input, _) = space0(input)?;
    let (input, value) = take_while(is_valid_numeric)(input)?;
    if value.len() == 0 {
        return Err(nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Eof,
        }));
    }
    Ok((input, value.to_owned()))
}

// search for #include and build debug info and keep track on branch prefix
fn include_line<'a>(input: &'a str) -> IResult<&'a str, VssType> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("#include")(input)?;
    let (input, _) = space1(input)?;
    let (input, filename) = vss_path(input)?;
    let (input, _) = space1(input)?;
    let (input, prefix) = opt(vss_path)(input)?;
    let (input, _) = eol(input)?;
    let include = VssInclude {
        filename: filename,
        prefix: prefix,
    };
    Ok((input, VssType::Include(include)))
}

// continuous lines are regroup into one single line
fn empty_line<'a>(input: &'a str) -> IResult<&'a str, VssType> {
    let (input, _) = eol(input)?;
    Ok((input, VssType::Empty()))
}

// comment line are removed, while keeping track of line vss_number
fn comment_line<'a>(input: &'a str) -> IResult<&'a str, VssType> {
    let (input, _) = space0(input)?;
    let (input, _) = sharp(input)?;
    let (input, value) = not_line_ending(input)?;
    let (input, _) = eol(input)?;
    Ok((input, VssType::Comment(value.to_string())))
}

// data line is anything with contend
fn data_line<'a>(input: &'a str) -> IResult<&'a str, VssType> {
    let (input, value) = not_line_ending(input)?;
    let (input, _) = eol(input)?;
    Ok((input, VssType::Data(value.to_string())))
}

// check for end of buffer
pub fn eof_data<'a>(input: &'a str) -> IResult<&'a str, VssType> {
    let _ = eof(input)?;
    Ok(("", VssType::Eof()))
}

// get line return a share enum for every class of line
pub fn get_line<'a>(input: &'a str, vss: &VssHandle) -> IResult<&'a str, ()> {
    let (input, line) = alt((empty_line, include_line, comment_line, data_line, eof_data))(input)?;
    vss.count.set(vss.count.get() + 1);
    match line {
        VssType::Include(include) => {
            let mut vss_incl = VssHandle::new(
                include.filename,
                Some(vss.filename.dirname.clone()),
                include.prefix,
            );
            vss_incl.data = vss.data.clone();

            match vss_from_file(&vss_incl) {
                Err(error) => {
                    return Err(nom::Err::Error(Error {
                        input: to_static_str(error.get_info()),
                        code: ErrorKind::Eof,
                    }))
                }
                Ok(_) => {}
            }
        }

        VssType::Data(text) => {
            let data = VssLine {
                line: vss.count.get(),
                text: text,
                filename: vss.filename.clone(),
            };
            let mut vss_data = vss.data.try_borrow_mut().unwrap();
            vss_data.lines.push(data);
        }

        // force an empty line at eof
        VssType::Eof() => {
            println!("Done file:{}", vss.filename.basename);
        }

        // remove empty lines
        VssType::Empty() => {}

        // remove comments
        VssType::Comment(_) => {}
    }
    Ok((input, ()))
}

fn check_indent(input: &str, idt_size: usize) -> IResult<&str, usize> {
    let (input, spaces) = take_while_m_n(0, idt_size, |c| c == ' ')(input)?;
    if idt_size != spaces.len() {
        let err = nom::Err::Error(Error {
            input: input,
            code: ErrorKind::Fail,
        });
        return Err(err);
    }
    Ok((input, spaces.len()))
}

// a label start at eol and is follow by ':'
fn vss_label(input: &str) -> IResult<&str, (String, usize)> {
    let (input, label) = argument(input)?;
    let (input, _) = colum(input)?;
    let (input, _) = eol(input)?;
    let (_, indent) = space1(input)?;
    Ok((input, (label.to_string(), indent.len())))
}

// encapsulate tag to simplify error handling with match
fn check_tag<'a>(input: &'a str, label: &str) -> IResult<&'a str, ()> {
    let (input, _) = tag(label)(input)?;
    Ok((input, ()))
}

// get line removing leading space
pub fn get_one_line(input: &str) -> IResult<&str, String> {
    let (input, _) = space0(input)?;
    let (input, value) = not_line_ending(input)?;
    Ok((input, value.to_string()))
}

// get data from an indented line
fn text_indent(input: &str, idt_size: usize) -> IResult<&str, String> {
    let (input, _) = check_indent(input, idt_size)?;
    let (input, value) = not_line_ending(input)?;
    let (input, _) = eol(input)?;
    Ok((input, value.to_string()))
}

// return a block of indented line as a vector or string
fn many_indent_lines(mut input: &str, idt_size: usize) -> IResult<&str, Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    loop {
        match text_indent(input, idt_size) {
            Ok((pointer, value)) => {
                input = pointer;
                result.push(value);
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok((input, result))
}

// search for a tag (word+':') within an indexed line
fn search_indent_tag<'a>(
    input: &'a str,
    label: &str,
    idt_size: usize,
) -> IResult<&'a str, (&'a str, usize)> {
    let mut next = input;
    loop {
        let (start, idt_new) = check_indent(next, idt_size)?;
        match check_tag(start, label) {
            Ok((input, _)) => {
                let (input, spaces) = space0(input)?;
                return Ok((input, (start, idt_new + label.len() + spaces.len())));
            }
            Err(_error) => {
                let (index, _) = not_line_ending(start)?;
                let (index, _) = eol(index)?;
                next = index;
            }
        }
    }
}

// return vss datatype info
fn vss_datatype(input: &str, idt_size: usize) -> IResult<&str, VssElement> {
    let label = "datatype:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = argument(input)?;
    let (input, _) = space0(input)?;
    let (input, array_opt) = opt(tag("[]"))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = eol(input)?;
    match VssValueType::from_str(value.as_str()) {
        Err(error) => Err(afb_to_nom_error(start, &error)),
        Ok(value) => {
            let is_array = match array_opt {
                Some(_) => true,
                None => false,
            };
            let data_type = VssDataType {
                is_type: value,
                is_array: is_array,
            };
            Ok((input, VssElement::DataType(data_type)))
        }
    }
}

// return vss object type info
fn vss_objtype(input: &str, idt_size: usize) -> IResult<&str, VssObjectType> {
    let label = "type:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = argument(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = eol(input)?;
    match VssObjectType::from_str(value.as_str()) {
        Err(error) => Err(afb_to_nom_error(start, &error)),
        Ok(value) => Ok((input, value)),
    }
}

fn vss_unit(input: &str, idt_size: usize) -> IResult<&str, VssElement> {
    let label = "unit:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = vss_path(input)?;
    let (input, _) = eol(input)?;
    match VssUnit::from_str(value.as_str()) {
        Err(error) => Err(afb_to_nom_error(start, &error)),
        Ok(value) => Ok((input, VssElement::ObjUnit(value))),
    }
}

fn vss_arraysize(input: &str, idt_size: usize) -> IResult<&str, VssElement> {
    let label = "arraysize:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = argument(input)?;
    let (input, _) = eol(input)?;
    match value.parse::<usize>() {
        Err(_error) => Err(nom::Err::Error(Error {
            input: start,
            code: ErrorKind::AlphaNumeric,
        })),
        Ok(value) => Ok((input, VssElement::DataArraySz(value))),
    }
}

fn vss_min(input: &str, idt_size: usize) -> IResult<&str, VssElement> {
    let label = "min:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = vss_number(input)?;
    let (input, _) = eol(input)?;
    match value.parse::<i64>() {
        Err(_error) => Err(nom::Err::Error(Error {
            input: start,
            code: ErrorKind::AlphaNumeric,
        })),
        Ok(value) => Ok((input, VssElement::DataMinVal(value))),
    }
}

fn vss_max(input: &str, idt_size: usize) -> IResult<&str, VssElement> {
    let label = "man:";
    let (input, (start, _)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = vss_number(input)?;
    let (input, _) = eol(input)?;
    match value.parse::<i64>() {
        Err(_error) => Err(nom::Err::Error(Error {
            input: start,
            code: ErrorKind::AlphaNumeric,
        })),
        Ok(value) => Ok((input, VssElement::DataMaxVal(value))),
    }
}

fn vss_aggregate<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    let label = "aggregate";
    let (input, _) = search_indent_tag(input, label, idt_size)?;
    let (input, _) = space0(input)?;
    let (input, value) = alt((tag_no_case("true"), tag_no_case("false")))(input)?;
    let (input, _) = eol(input)?;
    let result = if value != "true" { true } else { false };
    Ok((input, VssElement::ObjAggregate(result)))
}

fn indent_dash<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, ()> {
    let (input, _) = check_indent(input, idt_size)?;
    let (input, _) = dash(input)?;
    let (input, _) = space1(input)?;
    Ok((input, ()))
}

fn indent_string_or_number<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, String> {
    let (input, _) = check_indent(input, idt_size)?;
    let (input, value) = string_or_number(input)?;
    Ok((input, value))
}

// ["Left","Right"] [1,2] Prefix[val1,...valn]
fn get_one_instance<'a>(input: &'a str) -> IResult<&'a str, VssInstance> {
    let mut instance = VssInstance {
        prefix: None,
        array: Vec::new(),
    };
    let (input, _) = space0(input)?;
    let (input, prefix) = opt(alphanumeric1)(input)?;
    let (input, _) = space0(input)?;
    match prefix {
        None => {}
        Some(value) => instance.prefix = Some(value.to_string()),
    };
    let (mut input, _) = open_bracket(input)?;
    let input = loop {
        let (next, _) = opt(quote)(input)?;
        let (next, value) = opt(alphanumeric1)(next)?;
        let (next, _) = opt(quote)(next)?;
        match value {
            Some(data) => instance.array.push(data.to_string()),
            None => {}
        };
        let (next, value) = alt((close_bracket, comma))(next)?;
        let (next, _) = space0(next)?;
        if value == ']' {
            break next;
        };
        input = next;
    };
    Ok((input, instance))
}

fn vss_instances<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    let label = "instances:";
    let mut instances: Vec<VssInstance> = Vec::new();
    let (input, _) = search_indent_tag(input, label, idt_size)?;

    let (input, instance) = opt(get_one_instance)(input)?;
    let input = match instance {
        Some(value) => {
            instances.push(value);
            input
        }
        None => {
            let (input, _) = eol(input)?;
            let (_, spaces) = space1(input)?;
            let idt_size = spaces.len();

            let mut next = input;
            let next = loop {
                let input = match indent_dash(next, idt_size) {
                    Err(_error) => break next,
                    Ok((pointer, _)) => pointer,
                };
                let (input, instance) = get_one_instance(input)?;
                instances.push(instance);
                next = input;
            };
            next
        }
    };
    Ok((input, VssElement::ObjInstances(instances)))
}

// value | [value1,value2,...]
fn get_one_allowed<'a>(input: &'a str) -> IResult<&'a str, Vec<String>> {
    let (input, _) = space0(input)?;
    let mut allowed: Vec<String> = Vec::new();
    let (mut input, bracket) = opt(open_bracket)(input)?;
    let input = match bracket {
        None => {
            let (input, value) = string_or_number(input)?;
            allowed.push(value);
            input
        }

        Some(_) => loop {
            let (next, value) = opt(string_or_number)(input)?;
            match value {
                Some(data) => allowed.push(data.to_string()),
                None => {}
            };
            let (next, value) = alt((close_bracket, comma))(next)?;
            let (next, _) = space0(next)?;
            if value == ']' {
                break next;
            };
            input = next;
        },
    };
    Ok((input, allowed))
}

fn vss_array<'a>(input: &'a str, label: &str, idt_size: usize) -> IResult<&'a str, Vec<String>> {
    let (input, _) = search_indent_tag(input, label, idt_size)?;
    let mut allowed: Vec<String>;

    let (input, result) = opt(get_one_allowed)(input)?;
    let input = match result {
        Some(value) => {
            allowed = value;
            input
        }
        None => {
            allowed = Vec::new();
            let (input, _) = space0(input)?;
            let (input, _) = open_bracket(input)?;
            let (input, _) = space0(input)?;
            let (input, value) = opt(string_or_number)(input)?;
            let input = match value {
                None => input,
                Some(value) => {
                    allowed.push(value);
                    let (input, _) = space0(input)?;
                    let (input, _) = comma(input)?;
                    let (input, _) = not_line_ending(input)?;
                    input
                }
            };

            let (input, _) = eol(input)?;
            let (_, spaces) = space1(input)?;
            let idt_size = spaces.len();

            let mut next = input;
            let next = loop {
                let input = match indent_string_or_number(next, idt_size) {
                    Err(_error) => break next,
                    Ok((pointer, value)) => {
                        allowed.push(value);
                        pointer
                    }
                };
                let (input, _) = not_line_ending(input)?;
                let (input, _) = newline(input)?;
                next = input;
            };
            next
        }
    };
    Ok((input, allowed))
}

fn vss_allowed<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    let label = "allowed:";
    let (input, values) = vss_array(input, label, idt_size)?;
    Ok((input, VssElement::DataAllowed(values)))
}

fn vss_default<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    let label = "default:";
    let (input, values) = vss_array(input, label, idt_size)?;
    Ok((input, VssElement::DataDefault(values)))
}

// eat a block until indentation stop and check for empty new line
fn ignore_indent_block<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, ()> {
    let mut start = input;
    loop {
        match check_indent(start, idt_size) {
            Ok((input, _)) => {
                let (input, _) = not_line_ending(input)?;
                let (input, _) = eol(input)?;
                start = input;
            }
            Err(_) => break,
        };
    }
    let (input, _) = opt(eol)(start)?;
    Ok((input, ()))
}

fn get_block_indent<'a>(
    input: &'a str,
    label: &str,
    idt_size: usize,
) -> IResult<&'a str, VssElement> {
    let (input, (_, idt_new)) = search_indent_tag(input, label, idt_size)?;
    let (input, value) = not_line_ending(input)?;
    let (input, _) = eol(input)?;

    let (input, mut result) = many_indent_lines(input, idt_new)?;
    result.insert(0, value.to_string());

    Ok((input, VssElement::ObjDescription(result.join(" "))))
}

fn vss_description<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    get_block_indent(input, "description:", idt_size)
}
fn vss_comment<'a>(input: &'a str, idt_size: usize) -> IResult<&'a str, VssElement> {
    get_block_indent(input, "comment:", idt_size)
}

// equivalent to permutation with indentation support
fn get_indent_objects(
    input: &str,
    indent: usize,
    callbacks: Vec<IndentCallback>,
) -> IResult<&str, Vec<VssElement>> {
    let mut result: Vec<VssElement> = Vec::new();
    for callback in callbacks {
        match callback(input, indent) {
            Ok((_, elem)) => {
                result.push(elem);
            }
            Err(error) => {
                let (input, code) = nom_to_code_error(error);
                match code {
                    ErrorKind::Fail => {}
                    _ => {
                        return Err(nom::Err::Error(Error {
                            input: input,
                            code: ErrorKind::Verify,
                        }));
                    }
                }
            }
        }
    }

    Ok((input, result))
}

fn check_authorized_labels<'a>(
    mut start: &'a str,
    idt_size: usize,
    mut labels: Vec<&'static str>,
) -> IResult<&'a str, ()> {
    let mut defaults: Vec<&'static str> = vec!["type", "deprecation", "description", "comment"];
    labels.append(&mut defaults);

    let input = loop {
        let mut index = labels.iter();
        let input = match check_indent(start, idt_size) {
            Ok((input, _)) => input,
            Err(_) => break start,
        };

        let result = tuple((alphanumeric1, space0, colum))(input);
        let label = match result {
            Err(_) => {
                let (input, _) = not_line_ending(input)?;
                let (input, _) = newline(input)?;
                start = input;
                continue;
            }
            Ok((input, (label, _, _))) => {
                let (input, _) = not_line_ending(input)?;
                let (input, _) = newline(input)?;
                start = input;
                label
            }
        };

        match index.find(|&value| *value == label) {
            Some(_) => {}
            None => {
                eprintln!("parsing-error: unauthorized tag => '{}:'", label);
                return Err(nom::Err::Error(Error {
                    input: input,
                    code: ErrorKind::Satisfy,
                }));
            }
        }
    };

    Ok((input, ()))
}

fn vss_attribute<'a>(
    locator: &Locator,
    start: &'a str,
    label: String,
    vtype: VssObjectType,
    indent: usize,
) -> IResult<&'a str, VssObject> {
    let mut object = VssAttribute::new(locator, start, label, vtype);
    let (input, elements) = get_indent_objects(
        start,
        indent,
        vec![
            vss_description,
            vss_comment,
            vss_datatype,
            vss_unit,
            vss_default,
            vss_allowed,
        ],
    )?;

    for elem in elements {
        match elem {
            VssElement::DataType(data) => {
                object.datatype = data.is_type;
                if let None = object.arraysize {
                    if data.is_array {
                        object.arraysize = Some(0)
                    }
                };
            }
            VssElement::ObjUnit(data) => object.unit = data,
            VssElement::ObjDescription(data) => object.description = Some(data),
            VssElement::ObjComment(data) => object.comment = Some(data),
            VssElement::DataDefault(data) => object.default = data,
            VssElement::DataAllowed(data) => object.allowed = data,
            _ => {
                let _error = nom::Err::Error(Error {
                    input: input,
                    code: ErrorKind::Satisfy,
                });
            }
        }
    }
    check_authorized_labels(
        start,
        indent,
        vec!["arraysize", "datatype", "default", "allowed", "unit", "min", "max"],
    )?;

    Ok((input, VssObject::Attribute(object)))
}

fn vss_branch<'a>(
    locator: &Locator,
    start: &'a str,
    label: String,
    vtype: VssObjectType,
    indent: usize,
) -> IResult<&'a str, VssObject> {
    let mut object = VssBranch::new(locator, start, label, vtype);
    let (input, elements) = get_indent_objects(
        start,
        indent,
        vec![vss_description, vss_comment, vss_aggregate, vss_instances],
    )?;

    for elem in elements {
        match elem {
            VssElement::ObjDescription(data) => object.description = Some(data),
            VssElement::ObjComment(data) => object.comment = Some(data),
            VssElement::ObjAggregate(data) => object.aggregate = data,
            VssElement::ObjInstances(data) => object.instances = data,
            _ => {
                return Err(nom::Err::Error(Error {
                    input: input,
                    code: ErrorKind::Satisfy,
                }))
            }
        }
    }
    check_authorized_labels(start, indent, vec!["aggregate", "instances"])?;

    Ok((input, VssObject::Branch(object)))
}

fn vss_sensor<'a>(
    locator: &Locator,
    start: &'a str,
    label: String,
    vtype: VssObjectType,
    indent: usize,
) -> IResult<&'a str, VssObject> {
    let mut object = VssSensor::new(locator, start, label, vtype);
    let (input, elements) = get_indent_objects(
        start,
        indent,
        vec![
            vss_description,
            vss_comment,
            vss_datatype,
            vss_arraysize,
            vss_default,
            vss_allowed,
            vss_unit,
            vss_min,
            vss_max,
        ],
    )?;

    for elem in elements {
        match elem {
            VssElement::DataType(data) => {
                object.datatype = data.is_type;
                if let None = object.arraysize {
                    if data.is_array {
                        object.arraysize = Some(0)
                    }
                };
            }
            VssElement::ObjUnit(data) => object.unit = data,
            VssElement::ObjDescription(data) => object.description = Some(data),
            VssElement::ObjComment(data) => object.comment = Some(data),
            VssElement::DataMinVal(data) => object.min = Some(data),
            VssElement::DataMaxVal(data) => object.max = Some(data),
            VssElement::DataDefault(data) => object.default = data,
            VssElement::DataAllowed(data) => object.allowed = data,

            _ => {
                let _error = nom::Err::Error(Error {
                    input: "element incompatible with vss-sensor",
                    code: ErrorKind::Satisfy,
                });
            }
        }
    }

    check_authorized_labels(
        start,
        indent,
        vec!["arraysize", "datatype","default", "allowed", "unit", "min", "max"],
    )?;

    Ok((input, VssObject::Sensor(object)))
}

// search for vss object data with &str buffer
pub fn vss_object<'a>(locator: &Locator, input: &'a str) -> IResult<&'a str, VssObject> {
    let (start, (label, indent)) = vss_label(input)?;

    // extract prefix from locator
    let vss_data = locator.data.try_borrow().unwrap();
    let location = location(locator, start.len());
    let line = &vss_data.lines[location];
    let label = match &line.filename.prefix {
        Some(prefix) => format!("{}.{}", prefix.clone(), label),
        None => label,
    };

    // get object type
    let vtype = match vss_objtype(start, indent) {
        Ok((_pointer, elem)) => elem,
        Err(_error) => {
            let error = nom::Err::Error(Error {
                input: "vssobject type not set",
                code: ErrorKind::Satisfy,
            });
            return Err(error);
        }
    };

    let (input, object) = match vtype {
        VssObjectType::Attribute => vss_attribute(locator, start, label, vtype, indent)?,
        VssObjectType::Branch => vss_branch(locator, start, label, vtype, indent)?,
        VssObjectType::Sensor => vss_sensor(locator, start, label, vtype, indent)?,
        VssObjectType::Actuator => vss_sensor(locator, start, label, vtype, indent)?,
        VssObjectType::Unset => {
            panic!("(hoop) internal error object type not set")
        }
    };

    // indentation block processed let's move to next one
    let (input, _) = ignore_indent_block(input, indent)?;

    Ok((input, object))
}

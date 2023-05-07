/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 */

use std::fs::File;
use std::io;
use std::io::prelude::*;
use nom::error::{Error, ErrorKind};

use crate::types::*;
use crate::parser::*;

pub fn _to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

// make nom error to leverage ?; try method
pub fn afb_to_nom_error<'a>(input: &'a str, _error: &AfbError) -> nom::Err<Error<&'a str>> {
    nom::Err::Error(Error {
        input: input,
        code: ErrorKind::Verify,
    })
}

pub fn nom_to_afb_error(input: &str, error: nom::Err<Error<&str>>) -> AfbError {
    match error {
        nom::Err::Error(error) => {
            let info = match get_one_line(input) {
                Ok((_, info)) => format!("{}({})", error.to_string(), info),
                Err(_) => error.to_string(),
            };
            AfbError::new("parsing-error", info)
        }
        nom::Err::Incomplete(_error) => AfbError::new("parsing-error", "incomplete".to_string()),
        nom::Err::Failure(_error) => AfbError::new("parsing-error", "failure".to_string()),
    }
}

// loop on line of buffer until all attributes are parsed
pub fn vss_parse_rules<'a>(locator: &Locator) -> Result<VssSpec, AfbError> {
    let mut vss = VssSpec {
        attributes: Vec::new(),
        sensors: Vec::new(),
        branches: Vec::new(),
    };

    let mut input = locator.buffer.as_str();
    while input.len() > 0 {
        match vss_object(locator, input) {
            Ok((pointer, object)) => {
                //println!("*** vss obj:{:?}", object);
                match object {
                    VssObject::Attribute(obj) => vss.attributes.push(obj),
                    VssObject::Sensor(obj) => vss.sensors.push(obj),
                    VssObject::Branch(obj) => vss.branches.push(obj),
                }
                match eof_data(pointer) {
                    Ok(_) => break,
                    Err(_error) => {}
                }
                input = pointer;
            }
            Err(error) => {
                let afb_error = match error {
                    nom::Err::Error(error) => {
                        let vss_data = locator.data.try_borrow().unwrap();
                        let location = location(locator, error.input.len());
                        let line = &vss_data.lines[location];
                        let info = match get_one_line(error.input) {
                            Ok((_, info)) => info,
                            Err(_) => error.input.to_string(),
                        };
                        AfbError::new(
                            "parsing-error",
                            format!(
                                "{}{}:{} 'invalid: ({})'",
                                line.filename.dirname, line.filename.basename, line.line, info
                            ),
                        )
                    }
                    nom::Err::Incomplete(_error) => {
                        AfbError::new("parsing-error", "incomplete".to_string())
                    }
                    nom::Err::Failure(_error) => {
                        AfbError::new("parsing-error", "failure".to_string())
                    }
                };
                return Err(afb_error);
            }
        }
    }
    Ok(vss)
}


// read a file in RAM and transform its buffer as &str
pub fn vss_from_file(vss: &VssHandle) -> Result<(), AfbError> {
    let dbc_buffer = || -> Result<Vec<u8>, io::Error> {
        let fullname = format!("{}{}", vss.filename.dirname, vss.filename.basename);
        let mut fd = File::open(fullname)?;
        let size = fd.metadata().unwrap().len();
        let mut buffer = Vec::with_capacity(size as usize);

        fd.read_to_end(&mut buffer)?;
        Ok(buffer)
    };

    match dbc_buffer() {
        Err(error) => {
            let info = format!(
                "{}{} ({})",
                vss.filename.dirname,
                vss.filename.basename,
                error.to_string()
            );
            Err(AfbError::new("vss-open-fail", info))
        }
        Ok(buffer) => {
            let mut input = std::str::from_utf8(&buffer).unwrap();
            while input.len() > 0 {
                match get_line(input, &vss) {
                    Ok((reste, _)) => {
                        input = reste;
                    }
                    Err(error) => return Err(nom_to_afb_error(input, error)),
                };
            }

            Ok(())
        }
    }
}


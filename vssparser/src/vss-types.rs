/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 */

//use crate::utils::*;
use core::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::units::*;

#[derive(Debug)]
pub struct AfbError {
    uid: &'static str,
    info: String,
}

impl Clone for AfbError {
    fn clone(&self) -> AfbError {
        AfbError {
            uid: self.uid,
            info: self.info.to_owned(),
        }
    }
}

impl AfbError {
    pub fn to_str(&self) -> &'static str {
        let text = format!("uid:{} info:{}", self.uid, self.info);
        to_static_str(text)
    }
    pub fn new(uid: &'static str, info: String) -> Self {
        AfbError {
            uid: uid,
            info: info,
        }
    }
    pub fn get_info(&self) -> String {
        self.info.clone()
    }
}

pub fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

// search line number and filename from &str buffer index
pub fn location(locator: &Locator, tail: usize) -> usize {
    let head = if tail < locator.count {
        locator.count - tail
    } else {
        0
    };
    for idx in 0..locator.table.len() {
        if locator.table[idx] > head {
            return idx;
        }
    }
    return locator.table.len();
}

pub struct Filename {
    pub basename: String,
    pub dirname: String,
    pub prefix: Option<String>,
}

pub struct VssLine {
    pub line: u32,
    pub filename: Rc<Filename>,
    pub text: String,
}

pub struct VssData {
    pub lines: Vec<VssLine>,
}

pub struct VssHandle {
    pub count: Cell<u32>,
    pub filename: Rc<Filename>,
    pub data: Rc<RefCell<VssData>>,
}

impl VssHandle {
    pub fn new(filename: String, dirname: Option<String>, prefix: Option<String>) -> Self {
        // if absolute path then ignore dirname directory
        let dirname = if !filename.starts_with("/") {
            dirname
        } else {
            None
        };

        // rebuild current dirname + basename
        let mut path = filename.split("/").collect::<Vec<&str>>();
        let basename = path.pop().unwrap();
        let dirname = match dirname {
            Some(dirname) => dirname + "/" + path.join("/").as_str() + "/",
            None => {
                if path.len() > 0 {
                    path.join("/") + "/"
                } else {
                    "./".to_string()
                }
            }
        };

        VssHandle {
            count: Cell::new(0),
            filename: Rc::new(Filename {
                dirname: dirname,
                basename: basename.to_string(),
                prefix: prefix,
            }),
            data: Rc::new(RefCell::new(VssData { lines: Vec::new() })),
        }
    }
}

#[derive(Debug)]
pub struct VssInclude {
    pub filename: String,
    pub prefix: Option<String>,
}

pub enum VssType {
    Comment(String),
    Include(VssInclude),
    Data(String),
    Empty(),
    Eof(),
}

#[derive(Debug)]
pub enum VssObject {
    Branch(VssBranch),
    Sensor(VssSensor),
    Attribute(VssAttribute),
}

#[derive(PartialEq, Debug)]
pub enum VssObjectType {
    Branch,
    Sensor,
    Actuator,
    Attribute,
    Unset,
}

impl VssObjectType {
    pub fn from_str(value: &str) -> Result<Self, AfbError> {
        match value.to_lowercase().as_str() {
            "branch" => Ok(VssObjectType::Branch),
            "sensor" => Ok(VssObjectType::Sensor),
            "actuator" => Ok(VssObjectType::Actuator),
            "attribute" => Ok(VssObjectType::Attribute),
            _ => Err(AfbError {
                uid: "vss-objtype-invalid",
                info: format!("label:{} is not a vss object type", value),
            }),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            VssObjectType::Branch => "branch",
            VssObjectType::Sensor => "sensor",
            VssObjectType::Actuator => "actuator",
            VssObjectType::Attribute => "attribute",
            VssObjectType::Unset => "attribute",
        }
    }
}
#[derive(Debug)]
pub enum VssValueType {
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Boolean,
    Float,
    Double,
    String,
    Unset,
}

impl VssValueType {
    pub fn to_str(&self) -> &'static str {
        match self {
            VssValueType::Uint8 => "uint8",
            VssValueType::Int8 => "int8",
            VssValueType::Uint16 => "uint16",
            VssValueType::Int16 => "int16",
            VssValueType::Uint32 => "uint32",
            VssValueType::Int32 => "int32",
            VssValueType::Uint64 => "uint64",
            VssValueType::Int64 => "int64",
            VssValueType::Boolean => "boolean",
            VssValueType::Float => "float",
            VssValueType::Double => "double",
            VssValueType::String => "string",
            VssValueType::Unset => "unset",
        }
    }
    pub fn from_str(value: &str) -> Result<Self, AfbError> {
        match value.to_lowercase().as_str() {
            "uint8" => Ok(VssValueType::Uint8),
            "int8" => Ok(VssValueType::Int8),
            "uint16" => Ok(VssValueType::Uint16),
            "int16" => Ok(VssValueType::Int16),
            "uint32" => Ok(VssValueType::Uint32),
            "int32" => Ok(VssValueType::Int32),
            "uint64" => Ok(VssValueType::Uint64),
            "int64" => Ok(VssValueType::Int64),
            "boolean" => Ok(VssValueType::Boolean),
            "float" => Ok(VssValueType::Float),
            "double" => Ok(VssValueType::Double),
            "string" => Ok(VssValueType::String),
            _ => Err(AfbError {
                uid: "vss-datatype-invalid",
                info: format!("label:{} is not a vss data type", value),
            }),
        }
    }
}

pub struct VssDataType {
    pub is_type: VssValueType,
    pub is_array: bool,
}

pub enum VssElement {
    DataType(VssDataType),
    ObjType(VssObjectType),
    ObjUnit(VssUnit),
    ObjDescription(String),
    ObjComment(String),
    ObjAggregate(bool),
    ObjInstances(Vec<VssInstance>),
    DataAllowed(Vec<String>),
    DataDefault(Vec<String>),
    DataMinVal(i64),
    DataMaxVal(i64),
    DataArraySz(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VssDataValue {
    NotAvailable,
    Bool(bool),
    String(String),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
    Double(f64),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
    Int32Array(Vec<i32>),
    Int64Array(Vec<i64>),
    Uint32Array(Vec<u32>),
    Uint64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
}

#[derive(Debug)]
pub struct VssInstance {
    pub prefix: Option<String>,
    pub array: Vec<String>,
}

#[derive(Debug)]
pub struct VssBranch {
    pub vpath: String,
    pub vtype: VssObjectType,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub location: usize,
    pub instances: Vec<VssInstance>,
    pub aggregate: bool,
}

impl VssBranch {
    pub fn new(locator: &Locator, input: &str, label: String, vtype: VssObjectType) -> VssBranch {
        VssBranch {
            vpath: label,
            vtype: vtype,
            description: None,
            comment: None,
            aggregate: false,
            instances: Vec::new(),
            location: location(locator, input.len()),
        }
    }

    pub fn println(&self, locator: &Locator) {
        let vss_data = locator.data.try_borrow().unwrap();
        let line = &vss_data.lines[self.location];
        println!(
            "-- vpath: {}  ({}:{})",
            self.vpath, line.filename.basename, line.line
        );
        println!("   type: {}", self.vtype.to_str());
        if let Some(value) = &self.description {
            println!("   description: {}", value);
        }
        if let Some(value) = &self.comment {
            println!("   comment: {}", value);
        }
        println!("   agregate: {}", self.aggregate);

        if self.instances.len() > 0 {
            println!("   instance:");
            for instance in &self.instances {
                println!("     -- {:?}{:?}", instance.prefix, instance.array);
            }
        }
    }
}

#[derive(Debug)]
pub struct VssSensor {
    pub vpath: String,
    pub vtype: VssObjectType,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub datatype: VssValueType,
    pub arraysize: Option<usize>,
    pub default: Vec<String>,
    pub allowed: Vec<String>,
    pub location: usize,
    pub unit: VssUnit,
}

impl VssSensor {
    pub fn new(locator: &Locator, input: &str, label: String, vtype: VssObjectType) -> VssSensor {
        let vss_data = locator.data.try_borrow().unwrap();
        let location = location(locator, input.len());
        let line = &vss_data.lines[location];
        let vpath = match &line.filename.prefix {
            Some(prefix) => format!("{}.{}", prefix, label),
            None => label,
        };
        VssSensor {
            vpath: vpath,
            vtype: vtype,
            description: None,
            comment: None,
            default: Vec::new(),
            allowed: Vec::new(),
            datatype: VssValueType::Unset,
            unit: VssUnit::None,
            min: None,
            max: None,
            arraysize: None,
            location: location,
        }
    }

    pub fn println(&self, locator: &Locator) {
        let vss_data = locator.data.try_borrow().unwrap();
        let line = &vss_data.lines[self.location];
        println!(
            "-- vpath: {}  ({}:{})",
            self.vpath, line.filename.basename, line.line
        );
        println!("   type: {}", self.vtype.to_str());
        if let Some(value) = &self.description {
            println!("   description: {}", value);
        }
        if let Some(value) = &self.comment {
            println!("   comment: {}", value);
        }
        println!("   datatype: {}", self.datatype.to_str());
        if let Some(value) = &self.arraysize {
            println!("   arraysize: {}", value);
        }
        if self.default.len() > 0 {
            if self.default.len() == 1 {
                 println!("   default: {}", self.default[0]);
            } else {
                println!("   default:");
                for value in &self.default {
                    println!("     {},", value);
                }
            }
        }
        if self.allowed.len() > 0 {
            println!("   allowed:");
            for value in &self.allowed {
                println!("     {},", value);
            }
        }
        if let Some(value) = self.min {
            println!("   min: {}", value);
        }
        if let Some(value) = self.max {
            println!("   max: {}", value);
        }
    }
}

#[derive(Debug)]
pub struct VssAttribute {
    pub vpath: String,
    pub vtype: VssObjectType,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub location: usize,
    pub datatype: VssValueType,
    pub arraysize: Option<usize>,
    pub default: Vec<String>,
    pub allowed: Vec<String>,
    pub unit: VssUnit,
}

impl VssAttribute {
    pub fn new(
        locator: &Locator,
        input: &str,
        label: String,
        vtype: VssObjectType,
    ) -> VssAttribute {
        VssAttribute {
            vpath: label,
            vtype: vtype,
            description: None,
            comment: None,
            datatype: VssValueType::Unset,
            default: Vec::new(),
            allowed: Vec::new(),
            arraysize: None,
            unit: VssUnit::None,
            location: location(locator, input.len()),
        }
    }
    pub fn println(&self, locator: &Locator) {
        let vss_data = locator.data.try_borrow().unwrap();
        let line = &vss_data.lines[self.location];
        println!(
            "-- vpath: {}  ({}:{})",
            self.vpath, line.filename.basename, line.line
        );
        println!("   type: {}", self.vtype.to_str());
        if let Some(value) = &self.description {
            println!("   description: {}", value);
        }
        if let Some(value) = &self.comment {
            println!("   comment: {}", value);
        }
        if let Some(value) = &self.arraysize {
            println!("   arraysize: {}", value);
        }
        println!("   datatype: {}", self.datatype.to_str());

        if self.default.len() > 0 {
            if self.default.len() == 1 {
                 println!("   default: {}", self.default[0]);
            } else {
                println!("   default:");
                for value in &self.default {
                    println!("     {},", value);
                }
            }
        }

        if self.allowed.len() > 0 {
            println!("   allowed:");
            for value in &self.allowed {
                println!("     {},", value);
            }
        }
        println!("   unit: {}", self.unit.to_str());
    }
}

pub struct Locator {
    pub table: Vec<usize>,
    pub buffer: String,
    pub count: usize,
    pub data: Rc<RefCell<VssData>>,
}

impl Locator {
    pub fn new(vss: VssHandle) -> Result<Self, AfbError> {
        let mut locator = Locator {
            table: Vec::new(),
            count: 0,
            buffer: "".to_string(),
            data: vss.data.clone(),
        };

        // track location and group vector of strings into a single buffer from vector of string
        let vss_data = match vss.data.try_borrow() {
            Ok(data) => data,
            Err(_error) => {
                return Err(AfbError::new(
                    "vss-handle-mut",
                    "fail to get mutable handle".to_string(),
                ))
            }
        };

        for idx in 0..vss_data.lines.len() {
            let line = vss_data.lines[idx].text.as_str();
            locator.count += line.len() + 1;
            locator.table.push(locator.count);
            locator.buffer += line;
            locator.buffer += "\n";

            // let vline=  &vss_data.lines[idx];
            // println!("{}{}:{}/{} =>{}", vline.filename.dirname, vline.filename.basename, vss_data.lines[idx].line,locator.count,line);
        }
        //locator.buffer += "\n";
        Ok(locator)
    }
}

pub struct VssSpec {
    pub attributes: Vec<VssAttribute>,
    pub sensors: Vec<VssSensor>,
    pub branches: Vec<VssBranch>,
}

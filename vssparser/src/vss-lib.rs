/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 */

// ref: https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md
// https://blog.adamchalmers.com/nom-chars/

// reference: https://covesa.github.io/vehicle_signal_specification/rule_set/basics/
// - node/path
// - branch
// - instance: https://covesa.github.io/vehicle_signal_specification/rule_set/instances/

extern crate nom;

#[path = "./vss-utils.rs"]
mod utils;

#[path = "./vss-units.rs"]
mod units;

#[path = "./vss-types.rs"]
mod types;

#[path = "./vss-parser.rs"]
mod parser;

pub mod prelude {
    pub use crate::utils::*;
    pub use crate::parser::*;
    pub use crate::types::*;
    pub use crate::units::*;
}


/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 */

extern crate vssparser;

use std::env;
use vssparser::prelude::*;

fn main() -> Result<(), AfbError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(AfbError::new(
            "invalid-args-count",
            "filename missing".to_string(),
        ));
    }

    // recursively parse VSS files (Fulup:TBD group in a single api call)
    let vss = VssHandle::new (args[1].to_string(), None, None);
    vss_from_file(&vss)?;
    let locator= Locator::new(vss)?;

    // let vss_data = vss.data.try_borrow().unwrap();
    // for vss in &vss_data.lines {
    //     println!("{}{}:{} =>{}", vss.filename.dirname, vss.filename.basename, vss.line, vss.text);
    // }

    match vss_parse_rules(&locator) {
        Err(error) => Err(error),
        Ok(vss) => {
            println!("\n== Branches ===");
            for branch in vss.branches {
                branch.println(&locator);
            }
            println!("\n== Sensors ===");
            for sensor in vss.sensors {
                sensor.println(&locator);
            }
            println!("\n== Attributes ===");
            for attribute in vss.attributes {
                attribute.println(&locator);
            }
            Ok(())
        }
    }
}

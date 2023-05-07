/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
 * reference: (covesa) /vehicle_signal_specification/spec/units.yaml
 */

use crate::types::*;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum VssUnit {
    units,
    mm,
    cm,
    m,
    km,
    inch,
    km_h,
    m_s,
    m_sx2,
    cm_sx2,
    ml,
    l,
    cmx3,
    celsius,
    degrees,
    degrees_s,
    W,
    kW,
    PS,
    kWh,
    g,
    kg,
    lbs,
    V,
    A,
    Ah,
    ms,
    s,
    min,
    h,
    day,
    weeks,
    months,
    years,
    Timestamp,
    mbar,
    Pa,
    kPa,
    stars,
    g_s,
    g_km,
    kWh_100km,
    ml_100km,
    l_100km,
    l_h,
    mpg,
    N,
    Nm,
    rpm,
    Hz,
    ratio,
    percent,
    nm_km,
    dBm,
    kN,
    None,
}

impl VssUnit {
    pub fn from_str(value: &str) -> Result<Self, AfbError> {
        match value.to_lowercase().as_str() {
            "units" => Ok(VssUnit::units),
            "mm" => Ok(VssUnit::mm),
            "cm" => Ok(VssUnit::cm),
            "m" => Ok(VssUnit::m),
            "km" => Ok(VssUnit::km),
            "inch" => Ok(VssUnit::inch),
            "km/h" => Ok(VssUnit::km_h),
            "m/s" => Ok(VssUnit::m_s),
            "m/s^2" => Ok(VssUnit::m_sx2),
            "cm/s^2" => Ok(VssUnit::cm_sx2),
            "ml" => Ok(VssUnit::ml),
            "l" => Ok(VssUnit::l),
            "cm^3" => Ok(VssUnit::cmx3),
            "celsius" => Ok(VssUnit::celsius),
            "degrees" => Ok(VssUnit::degrees),
            "degrees/s" => Ok(VssUnit::degrees_s),
            "w" => Ok(VssUnit::W),
            "kw" => Ok(VssUnit::kW),
            "ps" => Ok(VssUnit::PS),
            "kwh" => Ok(VssUnit::kWh),
            "g" => Ok(VssUnit::g),
            "kg" => Ok(VssUnit::kg),
            "lbs" => Ok(VssUnit::lbs),
            "v" => Ok(VssUnit::V),
            "a" => Ok(VssUnit::A),
            "ah" => Ok(VssUnit::Ah),
            "ms" => Ok(VssUnit::ms),
            "s" => Ok(VssUnit::s),
            "min" => Ok(VssUnit::min),
            "h" => Ok(VssUnit::h),
            "day" => Ok(VssUnit::day),
            "weeks" => Ok(VssUnit::weeks),
            "months" => Ok(VssUnit::months),
            "years" => Ok(VssUnit::years),
            "timestamp" => Ok(VssUnit::Timestamp),
            "mbar" => Ok(VssUnit::mbar),
            "pa" => Ok(VssUnit::Pa),
            "kpa" => Ok(VssUnit::kPa),
            "stars" => Ok(VssUnit::stars),
            "g/s" => Ok(VssUnit::g_s),
            "g/km" => Ok(VssUnit::g_km),
            "kwh/100km" => Ok(VssUnit::kWh_100km),
            "ml/100km" => Ok(VssUnit::ml_100km),
            "l/100km" => Ok(VssUnit::l_100km),
            "l/h" => Ok(VssUnit::l_h),
            "mpg" => Ok(VssUnit::mpg),
            "n" => Ok(VssUnit::N),
            "nm" => Ok(VssUnit::Nm),
            "rpm" => Ok(VssUnit::rpm),
            "hz" => Ok(VssUnit::Hz),
            "ratio" => Ok(VssUnit::ratio),
            "percent" => Ok(VssUnit::percent),
            "nm/km" => Ok(VssUnit::nm_km),
            "dbm" => Ok(VssUnit::dBm),
            "kn" => Ok(VssUnit::kN),

            _ => Err(AfbError::new(
                "vss-objunit-invalid",
                format!("label:{} is not a vss unit", value),
            )),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            VssUnit::units => "units",
            VssUnit::mm => "mm",
            VssUnit::cm => "cm",
            VssUnit::m => "m",
            VssUnit::km => "km",
            VssUnit::inch => "inch",
            VssUnit::km_h => "km/h",
            VssUnit::m_s => "m/s",
            VssUnit::m_sx2 => "m/s^2",
            VssUnit::cm_sx2 => "cm/s^2",
            VssUnit::ml => "ml",
            VssUnit::l => "l",
            VssUnit::cmx3 => "cm^3",
            VssUnit::celsius => "celsius",
            VssUnit::degrees => "degrees",
            VssUnit::degrees_s => "degrees/s",
            VssUnit::W => "W",
            VssUnit::kW => "kW",
            VssUnit::PS => "PS",
            VssUnit::kWh => "kWh",
            VssUnit::g => "g",
            VssUnit::kg => "kg",
            VssUnit::lbs => "lbs",
            VssUnit::V => "V",
            VssUnit::A => "A",
            VssUnit::Ah => "Ah",
            VssUnit::ms => "ms",
            VssUnit::s => "s",
            VssUnit::min => "min",
            VssUnit::h => "h",
            VssUnit::day => "day",
            VssUnit::weeks => "weeks",
            VssUnit::months => "months",
            VssUnit::years => "years",
            VssUnit::Timestamp => "Timestamp",
            VssUnit::mbar => "mbar",
            VssUnit::Pa => "Pa",
            VssUnit::kPa => "kPa",
            VssUnit::stars => "stars",
            VssUnit::g_s => "g/s",
            VssUnit::g_km => "g/km",
            VssUnit::kWh_100km => "kWh/100km",
            VssUnit::ml_100km => "ml/100km",
            VssUnit::l_100km => "l/100km",
            VssUnit::l_h => "l/h",
            VssUnit::mpg => "mpg",
            VssUnit::N => "N",
            VssUnit::Nm => "Nm",
            VssUnit::rpm => "rpm",
            VssUnit::Hz => "Hz",
            VssUnit::ratio => "ratio",
            VssUnit::percent => "percent",
            VssUnit::nm_km => "nm/km",
            VssUnit::dBm => "dBm",
            VssUnit::kN => "kN",
            VssUnit::None => "Unset",
        }
    }
}

pub struct VssUnitInfo {
    pub uid: VssUnit,
    pub label: &'static str,
    pub description: &'static str,
    pub domain: VssUnitClass,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum VssUnitClass {
    acceleration,
    angle,
    angular_speed,
    distance,
    distance_volume,
    electric_charge,
    electric_current,
    electric_potential,
    energu_consumption,
    energy,
    flow,
    force,
    frequency,
    mass,
    mass_distance,
    mass_per_time,
    None,
    power,
    pressure,
    rating,
    relation,
    rotational_speed,
    speed,
    temperature,
    time,
    Torque,
    volume,
    volume_distance,
}

pub struct VssUnitPool {
    units: Vec<VssUnitInfo>,
}
impl VssUnitInfo {
    pub fn new(
        uid: VssUnit,
        label: &'static str,
        description: &'static str,
        domain: VssUnitClass,
    ) -> VssUnitInfo {
        VssUnitInfo {
            uid: uid,
            label: label,
            description: description,
            domain: domain,
        }
    }

    pub fn get_pool() -> VssUnitPool {
        let mut pool = VssUnitPool { units: Vec::new() };

        pool.units.push(VssUnitInfo::new(
            VssUnit::mm,
            "millimeter",
            "Distance measured in millimeters",
            VssUnitClass::distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::cm,
            "centimeter",
            "Distance measured in centimeters",
            VssUnitClass::distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::m,
            "meter",
            "Distance measured in meters",
            VssUnitClass::distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::km,
            "kilometer",
            "Distance measured in kilometers",
            VssUnitClass::distance,
        ));

        pool.units.push(VssUnitInfo::new(
            VssUnit::inch,
            "inch",
            "Distance measured in inches",
            VssUnitClass::distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::km_h,
            "kilometer per hour",
            "Speed measured in kilometers per hours",
            VssUnitClass::speed,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::m_s,
            "meters per second",
            "Speed measured in meters per second",
            VssUnitClass::speed,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::m_sx2,
            "meters per second squared",
            "Acceleration measured in meters per second squared",
            VssUnitClass::acceleration,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::cm_sx2,
            "centimeters per second squared",
            "Acceleration measured in centimeters per second squared",
            VssUnitClass::acceleration,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::ml,
            "milliliter",
            "Volume measured in milliliters",
            VssUnitClass::volume,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::l,
            "liter",
            "Volume measured in liters",
            VssUnitClass::volume,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::cmx3,
            "cubic centimeters",
            "Volume measured in cubic centimeters",
            VssUnitClass::volume,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::celsius,
            "degree celsius",
            "Temperature measured in degree celsius",
            VssUnitClass::temperature,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::degrees,
            "degree",
            "Angle measured in degrees",
            VssUnitClass::angle,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::degrees_s,
            "degree per second",
            "Angular speed measured in degrees per second",
            VssUnitClass::angular_speed,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::W,
            "watt",
            "Power measured in watts",
            VssUnitClass::power,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kW,
            "kilowatt",
            "Power measured in kilowatts",
            VssUnitClass::power,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::PS,
            "horsepower",
            "Power measured in horsepower",
            VssUnitClass::power,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kWh,
            "kilowatt hours",
            "Energy consumption measured in kilowatt hours",
            VssUnitClass::energu_consumption,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::g,
            "gram",
            "Mass measured in grams",
            VssUnitClass::mass,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kg,
            "kilogram",
            "Mass measured in kilograms",
            VssUnitClass::mass,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::lbs,
            "pound",
            "Mass measured in pounds",
            VssUnitClass::mass,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::V,
            "volt",
            "Electric potential measured in volts",
            VssUnitClass::electric_potential,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::A,
            "ampere",
            "Electric current measured in amperes",
            VssUnitClass::electric_current,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::Ah,
            "ampere hours",
            "Electric charge measured in ampere hours",
            VssUnitClass::electric_charge,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::ms,
            "millisecond",
            "Time measured in milliseconds",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::s,
            "second",
            "Time measured in seconds",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::min,
            "minute",
            "Time measured in minutes",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::h,
            "hour",
            "Time measured in hours",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::day,
            "days",
            "Time measured in days",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::weeks,
            "weeks",
            "Time measured in weeks",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::months,
            "months",
            "Time measured in months",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::years,
            "years",
            "Time measured in years",
            VssUnitClass::time,
        ));
        pool.units.push(VssUnitInfo::new(VssUnit::Timestamp
            , "Timestamp"
            , "Unix time is a system for describing a point in time. It is the number of seconds that have elapsed since the Unix epoch, excluding leap seconds."
            , VssUnitClass::time
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::mbar,
            "millibar",
            "Pressure measured in millibars",
            VssUnitClass::pressure,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::Pa,
            "pascal",
            "Pressure measured in pascal",
            VssUnitClass::pressure,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kPa,
            "kilopascal",
            "Pressure measured in kilopascal",
            VssUnitClass::pressure,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::stars,
            "stars",
            "Rating measured in stars",
            VssUnitClass::rating,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::g_s,
            "grams per second",
            "Mass per time measured in grams per second",
            VssUnitClass::mass_per_time,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::g_km,
            "grams per kilometer",
            "Mass per distance measured in grams per kilometers",
            VssUnitClass::mass_distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kWh_100km,
            "kilowatt hours per 100 kilometers",
            "Energy consumption per distance measured in kilowatt hours per 100 kilometers",
            VssUnitClass::energy,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::ml_100km,
            "milliliter per 100 kilometers",
            "Volume per distance measured in milliliters per 100 kilometers",
            VssUnitClass::volume_distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::l_100km,
            "liter per 100 kilometers",
            "Volume per distance measured in liters per 100 kilometers",
            VssUnitClass::volume_distance,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::l_h,
            "liter per hour",
            "Flow measured in liters per hour",
            VssUnitClass::flow,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::mpg,
            "miles per gallon",
            "Distance per volume measured in miles per gallon",
            VssUnitClass::distance_volume,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::N,
            "newton",
            "force measured in newton",
            VssUnitClass::force,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::Nm,
            "newton meter",
            "Torque measured in newton meters",
            VssUnitClass::Torque,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::rpm,
            "revolutions per minute",
            "Rotational speed measured in revolutions per minute",
            VssUnitClass::rotational_speed,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::Hz,
            "frequency",
            "frequency measured in hertz",
            VssUnitClass::frequency,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::ratio,
            "ratio",
            "relation measured as ratio",
            VssUnitClass::relation,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::percent,
            "percent",
            "relation measured in percent",
            VssUnitClass::relation,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::nm_km,
            "nano meter per kilometer",
            "nm_km",
            VssUnitClass::None,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::dBm,
            "decibel milliwatt",
            "Power level expressed in decibels with reference to one milliwatt",
            VssUnitClass::relation,
        ));
        pool.units.push(VssUnitInfo::new(
            VssUnit::kN,
            "kilo newton",
            "force measured in kilo newton",
            VssUnitClass::force,
        ));

        pool
    }
}

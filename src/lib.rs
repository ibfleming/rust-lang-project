#[derive(Debug, PartialEq)]
pub enum UnitType {
    Temperature,
    Time,
    Length,
    Mass,
    Currency,
}

#[derive(Debug, PartialEq)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, PartialEq)]
pub enum TimeUnits {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug, PartialEq)]
pub enum LengthUnits {
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
    Inches,
    Feet,
    Yards,
    Miles,
}

use TemperatureUnits::*;
use TimeUnits::*;
use LengthUnits::*;

/// Temperature Conversion Function
///
/// # Parameters
///
/// * `a` - From temperature value of conversion.
/// * `a_unit` - Unit of the `a` temperature value.
/// * `b` - To temperature value of conversion.
///  * `b_unit` - Unit of the 'b' temperature value.
/// 
/// # Returns
///
/// * `f64` - Final value of the conversion
///
/// ```
pub fn temperature_conversion(a: f64, from_unit: TemperatureUnits, to_unit: TemperatureUnits) -> f64 
{
    match from_unit {
        Celsius => {
            match to_unit {
                Celsius => {
                    println!("Both units are the same.");
                    return a;
                },
                Fahrenheit => return (a * 9.0 / 5.0) + 32.0,
                Kelvin => return a + 273.15,
            }
        },
        Fahrenheit => {
            match to_unit {
                Celsius => return (a - 32.0) * 5.0 / 9.0,
                Fahrenheit => {
                    println!("Both units are the same.");
                    return a;
                },
                Kelvin => return (a - 32.0) * 5.0 / 9.0 + 273.15,
            }
        },
        Kelvin => {
            match to_unit {
                Celsius => return a - 273.15,
                Fahrenheit => return (a - 273.15) * 9.0 / 5.0 + 32.0,
                Kelvin => {
                    println!("Both units are the same.");
                    return a;
                },
            }
        },
    }
}

/// Time Conversion Function
///
/// # Parameters
///
/// * `a` - From time value of conversion.
/// * `a_unit` - Unit of the `a` time value.
/// * `b` - To time value of conversion.
///  * `b_unit` - Unit of the 'b' time value.
/// 
/// # Returns
///
/// * `f64` - Final value of the conversion
///
/// ```
pub fn time_conversion(a: f64, from_unit: TimeUnits, to_unit: TimeUnits) -> f64
{
    match from_unit {
        Seconds => {
            match to_unit {
                Seconds => {
                    println!("Both units are the same.");
                    return a;
                },
                Minutes => return a / 60.0,
                Hours => return a / 3600.0,
                Days => return a / 86400.0,
                Weeks => return a / 604800.0,
                Months => return a / 2628000.0,
                Years => return a / 31536000.0,
            }
        },
        Minutes => {
            match to_unit {
                Seconds => return a * 60.0,
                Minutes => {
                    println!("Both units are the same.");
                    return a;
                },
                Hours => return a / 60.0,
                Days => return a / 1440.0,
                Weeks => return a / 10080.0,
                Months => return a / 43800.0,
                Years => return a / 525600.0,
            }
        },
        Hours => {
            match to_unit {
                Seconds => return a * 3600.0,
                Minutes => return a * 60.0,
                Hours => {
                    println!("Both units are the same.");
                    return a;
                },
                Days => return a / 24.0,
                Weeks => return a / 168.0,
                Months => return a / 730.0,
                Years => return a / 8760.0,
            }
        },
        Days => {
            match to_unit {
                Seconds => return a * 86400.0,
                Minutes => return a * 1440.0,
                Hours => return a * 24.0,
                Days => {
                    println!("Both units are the same.");
                    return a;
                },
                Weeks => return a / 7.0,
                Months => return a / 30.0,
                Years => return a / 365.0,
            }
        },
        Weeks => {
            match to_unit {
                Seconds => return a * 604800.0,
                Minutes => return a * 10080.0,
                Hours => return a * 168.0,
                Days => return a * 7.0,
                Weeks => {
                    println!("Both units are the same.");
                    return a;
                },
                Months => return a / 4.34524,
                Years => return a / 52.1775,
            }
        },
        Months => {
            match to_unit {
                Seconds => return a * 2628000.0,
                Minutes => return a * 43800.0,
                Hours => return a * 730.0,
                Days => return a * 30.0,
                Weeks => return a * 4.34524,
                Months => {
                    println!("Both units are the same.");
                    return a;
                },
                Years => return a / 12.0,
            }
        },
        Years => {
            match to_unit {
                Seconds => return a * 31536000.0,
                Minutes => return a * 525600.0,
                Hours => return a * 8760.0,
                Days => return a * 365.0,
                Weeks => return a * 52.1775,
                Months => return a * 12.0,
                Years => {
                    println!("Both units are the same.");
                    return a;
                },
            }
        },
    }
}

/// Length Conversion Function
///
/// # Parameters
///
/// * `a` - From length value of conversion.
/// * `a_unit` - Unit of the `a` length value.
/// * `b` - To length value of conversion.
///  * `b_unit` - Unit of the 'b' length value.
/// 
/// # Returns
///
/// * `f64` - Final value of the conversion
///
/// '''
pub fn length_conversion(a: f64, from_unit: LengthUnits, to_unit: LengthUnits) -> f64
{
    match from_unit {
        Millimeters => {
            match to_unit {
                Millimeters => {
                    println!("Both units are the same.");
                    return a;
                },
                Centimeters => return a / 10.0,
                Meters => return a / 1000.0,
                Kilometers => return a / 1000000.0,
                Inches => return a / 25.4,
                Feet => return a / 304.8,
                Yards => return a / 914.4,
                Miles => return a / 1609344.0,
            }
        },
        Centimeters => {
            match to_unit {
                Millimeters => return a * 10.0,
                Centimeters => {
                    println!("Both units are the same.");
                    return a;
                },
                Meters => return a / 100.0,
                Kilometers => return a / 100000.0,
                Inches => return a / 2.54,
                Feet => return a / 30.48,
                Yards => return a / 91.44,
                Miles => return a / 160934.4,
            }
        },
        Meters => {
            match to_unit {
                Millimeters => return a * 1000.0,
                Centimeters => return a * 100.0,
                Meters => {
                    println!("Both units are the same.");
                    return a;
                },
                Kilometers => return a / 1000.0,
                Inches => return a * 39.3701,
                Feet => return a * 3.28084,
                Yards => return a * 1.09361,
                Miles => return a / 1609.34,
            }
        },
        Kilometers => {
            match to_unit {
                Millimeters => return a * 1000000.0,
                Centimeters => return a * 100000.0,
                Meters => return a * 1000.0,
                Kilometers => {
                    println!("Both units are the same.");
                    return a;
                },
                Inches => return a * 39370.1,
                Feet => return a * 3280.84,
                Yards => return a * 1093.61,
                Miles => return a / 1.60934,
            }
        },
        Inches => {
            match to_unit {
                Millimeters => return a * 25.0,
                Centimeters => return a * 2.54,
                Meters => return a / 39.3701,
                Kilometers => return a / 39370.1,
                Inches => {
                    println!("Both units are the same.");
                    return a;
                },
                Feet => return a / 12.0,
                Yards => return a / 36.0,
                Miles => return a / 63360.0,
            }
        },
        Feet => {
            match to_unit {
                Millimeters => return a * 304.8,
                Centimeters => return a * 30.48,
                Meters => return a / 3.28084,
                Kilometers => return a / 3280.84,
                Inches => return a * 12.0,
                Feet => {
                    println!("Both units are the same.");
                    return a;
                },
                Yards => return a / 3.0,
                Miles => return a / 5280.0,
            }
        },
        Yards => {
            match to_unit {
                Millimeters => return a * 914.4,
                Centimeters => return a * 91.44,
                Meters => return a / 1.09361,
                Kilometers => return a / 1093.61,
                Inches => return a * 36.0,
                Feet => return a * 3.0,
                Yards => {
                    println!("Both units are the same.");
                    return a;
                },
                Miles => return a / 1760.0,
            }
        },
        Miles => {
            match to_unit {
                Millimeters => return a * 1609344.0,
                Centimeters => return a * 160934.4,
                Meters => return a * 1609.34,
                Kilometers => return a * 1.60934,
                Inches => return a * 63360.0,
                Feet => return a * 5280.0,
                Yards => return a * 1760.0,
                Miles => {
                    println!("Both units are the same.");
                    return a;
                },
            }
        }
    }
}


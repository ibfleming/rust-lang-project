pub enum UnitType {
    Temperature,
    Time,
    Length,
    Mass,
    Currency,
}

pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub enum TimeUnits {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

use TemperatureUnits::*;
use TimeUnits::*;

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
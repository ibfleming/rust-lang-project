/*

    Unit Converter

    Temperature:
    Celius <-> Fahrenheit <-> Kelvin

    Time:
    Seconds <-> Minutes <-> Hours <-> Days <-> Weeks <-> Months <-> Years

    Length:
    Millimeters <-> Centimeters <-> Meters <-> Kilometers <-> Inches <-> Feet <-> Yards <-> Miles

    Mass:
    Milligrams <-> Grams <-> Kilograms <-> Ounces <-> Pounds <-> Stones

    US Dollar:
    Cents <-> Dollars

    Currency:
    Dollars <-> Euros <-> Pounds <-> Yen <-> Yuan <-> Rupees <-> Rubles <-> Pesos <-> Ringgit <-> Rupees

*/

enum UnitType {
    Temperature,
    Time,
    Length,
    Mass,
    Currency,
}

enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

use UnitType::*;
use Unit::*;

fn main() {

}

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
fn temperature_conversion(a: f64, from_unit: Unit, to_unit: Unit) -> f64 {

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
        Fahrenheit => todo!(),
        Kelvin => todo!(),
    }

    return 0.0;
}
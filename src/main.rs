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

use unit_converter::*;
use unit_converter::TemperatureUnits::*;

fn main() {
    temperature_conversion(64.0, Fahrenheit, Kelvin);
}
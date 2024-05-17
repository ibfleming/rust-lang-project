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
use unit_converter::UnitType::*;
use unit_converter::TemperatureUnits::*;
use std::io::{self, Write};

const DEGREE_SYMBOL: char = '\u{00B0}';

fn main() {

    // Input
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let input: Vec<&str> = input.split_whitespace().collect();
    if input.len() != 3 { return }
    
    if is_f64(input[0]) {

        let val = input[0].parse::<f64>().unwrap();
        let from_unit = is_valid_unit(input[1]);
        let to_unit = is_valid_unit(input[2]);

        if from_unit == None || to_unit == None { return }
        if from_unit != to_unit { return }

        //println!("{:?} {:?} {:?}", val, from_unit.unwrap(), to_unit.unwrap());

        match from_unit {
            Some(Temperature) => {
                let from_unit = get_temp_unit(input[1]);
                let to_unit = get_temp_unit(input[2]);
                if from_unit == None {
                    println!("Invalid Left unit.");
                    return;
                }
                if to_unit == None {
                    println!("Invalid Right unit.");
                    return;
                }
                match to_unit {
                    Some(Celsius) => println!("{:.2}{}C", temperature_conversion(val, from_unit.unwrap(), to_unit.unwrap()), DEGREE_SYMBOL),
                    Some(Fahrenheit) => println!("{:.2}{}F", temperature_conversion(val, from_unit.unwrap(), to_unit.unwrap()), DEGREE_SYMBOL),
                    Some(Kelvin) => println!("{:.2} K", temperature_conversion(val, from_unit.unwrap(), to_unit.unwrap())),
                    _ => return,
                }
            },
            Some(Time) => {
            },
            Some(Length) => {
            },
            Some(Currency) => {
            },
            Some(Mass) => {
            },
            None => return,
        }

    }
}

fn is_f64(x: &str) -> bool {
    match x.parse::<f64>() {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

fn is_valid_unit(s: &str) -> Option<UnitType> {
    match s.to_lowercase().as_str() {
        "c" | "celsius" => return Some(Temperature),
        "f" | "fahrenheit" => return Some(Temperature),
        "k" | "kelvin" => return Some(Temperature),
        "s" | "seconds" => return Some(Time),
        "min" | "minutes" => return Some(Time),
        "h" | "hours" => return Some(Time),
        "d" | "days" => return Some(Time),
        "w" | "weeks" => return Some(Time),
        "mo" | "months" => return Some(Time),
        "y" | "years" => return Some(Time),
        "mm" | "millimeters" => return Some(Length),
        "cm" | "centimeters" => return Some(Length),
        "m" | "meters" => return Some(Length),
        "km" | "kilometers" => return Some(Length),
        "in" | "inches" => return Some(Length),
        "ft" | "feet" => return Some(Length),
        "yd" | "yards" => return Some(Length),
        "mi" | "miles" => return Some(Length),
        _ => None,
    }
}

fn get_temp_unit(s: &str) -> Option<TemperatureUnits> {
    match s.to_lowercase().as_str() {
        "c" | "celsius" => return Some(Celsius),
        "f" | "fahrenheit" => return Some(Fahrenheit),
        "k" | "kelvin" => return Some(Kelvin),
        _ => None,
    }
}


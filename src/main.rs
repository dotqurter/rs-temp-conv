#[derive(PartialEq, Debug)]
enum Measure {
    Fahrenheit,
    Celsius,
    Kelvin,
}

fn main() {
    println!("Convert from:");
    let mut fromstr = String::new();
    std::io::stdin().read_line(&mut fromstr).expect("Failed to read line");

    println!("Convert to:");
    let mut tostr = String::new();
    std::io::stdin().read_line(&mut tostr).expect("Failed to read line");

    let lconv = get_conversion(fromstr.clone());
    let rconv = get_conversion(tostr);

    let lval = get_numbers(fromstr.clone());
    let res: f32;
    if lconv != rconv {
        match lconv {
            Measure::Fahrenheit => {
                res = match rconv {
                    Measure::Celsius => to_celsius(lval),
                    Measure::Kelvin => to_kelvin( to_celsius(lval) ),
                    _ => std::panic!(),
                };
            },
            Measure::Celsius => {
                res = match rconv {
                    Measure::Fahrenheit => to_fahrenheit(lval),
                    Measure::Kelvin => to_kelvin(lval),
                    _ => std::panic!(),
                };
            },
            Measure::Kelvin => {
                res = match rconv {
                    Measure::Celsius => lval - 273.15,
                    Measure::Fahrenheit  => to_fahrenheit(lval - 273.15),
                    _ => std::panic!(),
                };
            },
        }
    } else {
        res = lval;
    }
    println!("{} {:?}", res, rconv);
}
 
fn get_conversion(input: String) -> Measure {
    return match input.to_lowercase() {
        _w if _w.contains("f") => Measure::Fahrenheit,
        _w if _w.contains("c") => Measure::Celsius,
        _w if _w.contains("k") => Measure::Kelvin,
        String {..} => std::unimplemented!(),
    }
}

fn get_numbers(input: String) -> f32 {
    let _t: String = input.chars().filter( |&c| c.is_numeric() || c == '.').collect();
    return _t.parse::<f32>().unwrap();
}

/// Assumes Celsius -> Fahrenheit
fn to_fahrenheit(inp: f32) -> f32 {
    (1.8 * inp) + 32.0
}

/// Assumes Fahrenheit -> Celsius
fn to_celsius(inp: f32) -> f32 {
    (inp - 32.0) / 1.8
}

/// Assumes Celsius -> Kelvin
fn to_kelvin(inp: f32) -> f32 {
    inp + 273.15
}

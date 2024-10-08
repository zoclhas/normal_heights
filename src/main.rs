use clap::{crate_version, App, Arg};
use std::error::Error;
use std::str::FromStr;

use normal_heights::{map_normals_with_strength, DEFAULT_STRENGTH};

fn main() -> Result<(), Box<dyn Error>> {
    let default_strength = DEFAULT_STRENGTH.to_string();
    let matches = App::new("Normal Heights")
        .version(crate_version!())
        .author("Jon Olin")
        .about("Makes normal maps from height maps.")
        .arg(
            Arg::with_name("INPUT")
                .help("Input height map image file.")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Filename to write the normal map to.")
                .required(true),
        )
        .arg(
            Arg::with_name("strength")
                .help("Strength of the resulting normal map.")
                .next_line_help(true)
                .short("s")
                .long("strength")
                .default_value(&default_strength)
                .validator(strength_validator),
        )
        .arg(
            Arg::with_name("directx")
                .help("Generate DirectX normals (Inverted Y)")
                .next_line_help(true)
                .short("d")
                .long("directx")
                .default_value(&"false")
                .validator(directx_validator),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    let strength = f32::from_str(matches.value_of("strength").unwrap())?;
    let directx = bool::from_str(matches.value_of("directx").unwrap())?;

    let img = image::open(input)?;

    let normal_map = map_normals_with_strength(&img, strength, directx);

    normal_map.save(output)?;

    Ok(())
}

fn strength_validator(val: String) -> Result<(), String> {
    match f32::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "'{}' is not a valid number. Please use a base 10 number such as '6' or '3.14'.",
            val
        )),
    }
}

fn directx_validator(val: String) -> Result<(), String> {
    match bool::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "'{}' is not a valid option. Please use true or false.",
            val
        )),
    }
}

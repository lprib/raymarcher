use clap::{App, SubCommand, Arg, AppSettings};

mod vec3;
mod render_3d;
mod render_2d;

fn main() {
    let matches = App::new("Marcher")
        .version("0.1")
        .author("Liam Pribis <jackpribis@gmail.com>")
        .about("Julia Set Raymarcher")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("3d")
            .about("Render 3d julia set in window")
            .arg(Arg::with_name("width")
                .short("w")
                .long("width")
                .help("width of framebuffer")
                .takes_value(true)
                .required(true)
                .validator(positive_int_validator)
            )
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .help("height of framebuffer")
                .takes_value(true)
                .required(true)
                .validator(positive_int_validator)
            )
            .arg(Arg::with_name("float")
                .short("c")
                .help("c value of julia set")
                .required(true)
                .multiple(true)
                .number_of_values(4)
                .require_delimiter(true)
                .value_delimiter(",")
                .validator(float_validator)
            )
        )
        .get_matches();

    render_3d::main(256, 256, Default::default());
}

fn positive_int_validator(input: String) -> Result<(), String> {
    let int = input.parse::<u32>().map_err(|e| "must be a valid integer")?;
    if int > 0 {
        Ok(())
    } else {
        Err(String::from("Must be greater than zero"))
    }
}

fn float_validator(input: String) -> Result<(), String> {
    Ok(input.parse::<f64>().map(|_| ()).map_err(|_| "must be a valid float")?)
}
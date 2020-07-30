use std::str::FromStr;
use clap::{App, SubCommand, Arg, AppSettings, Values};
use crate::vec3::Vec3;
use crate::render_3d::raymarcher::RayMarcherConfig;
use cgmath::Quaternion;
use crate::render_3d::fractals::{Julia, Mandelbulb};
use crate::render_3d::scene_object::Sphere;

mod vec3;
mod render_3d;
// mod render_2d;

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
                .display_order(0)
                .takes_value(true)
                .required(true)
                .validator(positive_int_validator)
            )
            .arg(Arg::with_name("height")
                .short("h")
                .long("height")
                .help("height of framebuffer")
                .display_order(1)
                .takes_value(true)
                .required(true)
                .validator(positive_int_validator)
            )
            .arg(Arg::with_name("c")
                .short("c")
                .long("c")
                .help("c value of julia set")
                .display_order(2)
                .require_equals(true)
                .required(true)
                .multiple(true)
                .number_of_values(4)
                .require_delimiter(true)
                .value_delimiter(",")
                .value_names(&["cw", "cx", "cy", "cz"])
                .validator(float_validator)
            )
            .arg(optional_vec3_arg(
                "camera-pos",
                "position of camera in 3d space",
                "2,4,4",
                false,
            ))
            .arg(optional_vec3_arg(
                "look-at",
                "position to point camera towards in 3d space",
                "0,0,0",
                false,
            ))
            .arg(optional_vec3_arg(
                "light-pos",
                "position of light in 3d space",
                "2,4,4",
                false,
            ))
            .arg(optional_vec3_arg(
                "bg-color",
                "normalized (each element in [0, 1]) color of background",
                "0,0,0",
                true,
            ))
            .arg(optional_vec3_arg(
                "backplane",
                "values of x/y/z where rays will be assumed to be a miss (ie. back clipping planes)",
                "3,3,3",
                false,
            ))
            .arg(optional_vec3_arg(
                "specular-color",
                "normalized specular highlight color of the render",
                "1,1,1",
                true,
            ))
            .arg(optional_vec3_arg(
                "object-color",
                "normalized ambient color of the julia set",
                "0.8,0,0",
                true,
            ))
            .arg(Arg::with_name("zoom")
                .short("z")
                .long("zoom")
                .help("camera zoom")
                .takes_value(true)
                .default_value("1")
                .validator(float_validator)
            )
            .arg(Arg::with_name("aa-level")
                .long("aa-level")
                .help("level of anti-aliasing. --aa-level 2 will provide a 2x2 subpixel grid")
                .takes_value(true)
                .default_value("2")
                .validator(positive_int_validator)
            )
            .arg(Arg::with_name("specular-shininess")
                .long("specular-shininess")
                .help("Phong shininess value used when calculating specular highlights")
                .takes_value(true)
                .default_value("50")
                .validator(positive_float_validator)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("3d") {
        // all unwrapping should be OK because all args have validators and default values
        let config = RayMarcherConfig {
            camera_pos: matches.values_of("camera-pos").into(),
            look_at: matches.values_of("look-at").into(),
            light_pos: matches.values_of("light-pos").into(),
            background_color: matches.values_of("bg-color").into(),
            camera_zoom: matches.value_of("zoom").into_f64(),
            anti_aliasing_level: matches.value_of("aa-level").into_u32(),
            backplane_positions: matches.values_of("backplane").into(),
            specular_shininess: matches.value_of("specular-shininess").into_f64(),
            specular_color: matches.values_of("specular-color").into(),
            ..Default::default()
        };

        println!("{:#?}", config);

        let mut c = matches.values_of("c").unwrap();
        let c = Quaternion::new(
            c.next().into_f64(),
            c.next().into_f64(),
            c.next().into_f64(),
            c.next().into_f64(),
        );

        let width = matches.value_of("width").into_u32() as usize;
        let height = matches.value_of("height").into_u32() as usize;
        
        let object = Julia {
            color: matches.values_of("object-color").into(),
            c
        };

        // let object = Mandelbulb {
        //     color: matches.values_of("object-color").into()
        // };
        //
        // let object = Sphere {
        //     radius: 1.0,
        //     center: (0, 0, 0).into(),
        //     color: (1, 0, 0).into()
        // };

        render_3d::main(width, height, config, object);
    }
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

fn positive_float_validator(input: String) -> Result<(), String> {
    let f = input.parse::<f64>().map_err(|_| "must be a valid float")?;
    if f > 0.0 {
        Ok(())
    } else {
        Err(String::from("value must be greater than zero"))
    }
}

impl<'a> From<Option<Values<'a>>> for Vec3 {
    fn from(values: Option<Values<'a>>) -> Self {
        let mut values = values.unwrap();
        Vec3 {
            x: f64::from_str(values.next().unwrap()).unwrap(),
            y: f64::from_str(values.next().unwrap()).unwrap(),
            z: f64::from_str(values.next().unwrap()).unwrap(),
        }
    }
}

trait ArgumentConversion {
    fn into_f64(self) -> f64;
    fn into_u32(self) -> u32;
}

impl ArgumentConversion for Option<&str> {
    fn into_f64(self) -> f64 {
        f64::from_str(self.unwrap()).unwrap()
    }

    fn into_u32(self) -> u32 {
        u32::from_str(self.unwrap()).unwrap()
    }
}

fn optional_vec3_arg(name: &'static str, help: &'static str, default: &'static str, is_color: bool) -> Arg<'static, 'static> {
    Arg::with_name(name)
        .long(name)
        .help(help)
        .allow_hyphen_values(true)
        .require_equals(true)
        .multiple(true)
        .number_of_values(3)
        .require_delimiter(true)
        .value_delimiter(",")
        .value_names(if is_color {
            &["r", "g", "b"]
        } else {
            &["x", "y", "z"]
        })
        .default_value(default)
        .validator(float_validator)
}
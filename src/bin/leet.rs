extern crate handlebars;

use handlebars::{to_json, Handlebars};
use serde_json::value::{Map, Value as Json};
use std::{env, error::Error, fs::File, process};

pub struct Config {
    pub index: String,
    pub func: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!");
        }

        let index = args[1].clone();
        let func = args[2].clone();

        Ok(Config { index, func })
    }
}

pub fn make_data(config: &Config) -> Map<String, Json> {
    let mut data = Map::new();
    data.insert("func".to_string(), to_json(&config.func));
    data.insert("num".to_string(), to_json(&config.index));
    data
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    let data = make_data(&config);

    handlebars
        .register_template_file("template", "./templates/leetcode/template.hbs")
        .unwrap();

    let path = format!("src/leetcode/leetcode_{}.rs", config.index);
    let mut output_file = File::create(&path)?;
    handlebars.render_to_write("template", &data, &mut output_file)?;

    let lib_file = File::options()
        .append(true)
        .open("./src/leetcode/mod.rs")
        .unwrap();
    handlebars
        .register_template_string("export", "pub mod leetcode_{{num}};\n")
        .unwrap();
    handlebars
        .render_to_write("export", &data, &lib_file)
        .unwrap();

    println!("✅ Generated file: {}", &path);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("🚨 Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("🎯 Leetcode {}: {}", config.index, config.func);

    if let Err(e) = run(config) {
        println!("🚨 Application error: {e}");
        process::exit(1);
    }
}

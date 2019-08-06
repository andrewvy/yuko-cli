extern crate config;
extern crate serde;
extern crate reqwest;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;

mod settings;
mod api;

use std::collections::HashMap;

use settings::Settings;
use api::API;

use clap::Arg;

#[derive(Debug)]
enum Error {
    Config(config::ConfigError),
}

#[derive(Debug, Deserialize)]
struct Message {
    id: String,
    text: String
}

#[derive(Debug, Deserialize)]
struct Messages {
    messages: Vec<Message>
}

static DEFAULT_CONFIG_FILE: &'static str = "~/.config/yuko/config.json";

fn list(api_client: &API, query: &str) -> Result<Messages, ()> {
    let mut args = HashMap::new();

    args.insert("q".to_string(), query.to_owned());

    let mut response = api_client.get("messages", args).unwrap();

    match response.json::<Messages>() {
        Ok(json) => {
            for message in &json.messages {
                println!("{:?}", message);
            }

            Ok(json)
        },
        err => {
            println!("{:?}", err);

            Err(())
        }
    }
}

fn post(api_client: &API, text: &str) -> Result<Message, ()> {
    let mut args = HashMap::new();

    args.insert("text".to_string(), text.to_owned());

    let mut response = api_client.post("messages", args).unwrap();

    match response.json::<Message>() {
        Ok(json) => {
            println!("{:?}", json);

            Ok(json)
        },
        err => {
            println!("{:?}", err);

            Err(())
        }
    }
}

fn main() {
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(clap::Arg::with_name("config")
             .short("c")
             .long("config")
             .takes_value(true)
             .env("YUKO_CONFIG_FILE")
             .default_value(DEFAULT_CONFIG_FILE)
             .help("Specify a custom config file to load"))
        .subcommand(clap::SubCommand::with_name("list")
                    .arg(Arg::with_name("query")
                                 .required(true)
                                 .takes_value(true)
                                 .multiple(true)
                                 .help("Search query")))
        .subcommand(clap::SubCommand::with_name("post")
                    .arg(Arg::with_name("text")
                                 .required(true)
                                 .takes_value(true)
                                 .multiple(true)
                                 .index(1)
                                 .help("Message text to create.")))
        .get_matches();

    let config_file = matches.value_of("config").unwrap();

    match Settings::new(config_file) {
        Ok(settings) => {
            let api_client = API::new(&settings.api.token);

            match matches.subcommand() {
                (("list"), Some(m)) => {
                    match m.value_of("query") {
                        Some(text) => {
                            list(&api_client, &text);
                        },
                        _ => {}
                    }
                },
                (("post"), Some(m)) => {
                    match m.value_of("text") {
                        Some(text) => {
                            post(&api_client, &text);
                        },
                        _ => {}
                    }
                },
                _ => {
                    println!("Unknown command");
                }
            }
        },
        Err(err) => {
            println!("Could not find config file at {}, {}", DEFAULT_CONFIG_FILE, err);
        }
    };
}

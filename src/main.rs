use std::{env, process, thread};
use std::error::Error;

use clap::{App, Arg};

use crate::stock_api::Api;
use tokio::time::Duration;
use std::time::SystemTime;
use std::ops::Add;

mod stock_api;
mod email;

#[tokio::main]
async fn main() {
    let number_validator = |v: String| {
        v.parse::<i32>().unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });
        Ok(())
    };
    let allowed_values: [&str; 1] = ["BTC-USD"];
    let matches = App::new("stock-fetcher")
        .version("1.0.0")
        .about("fetch cryptocurrency stock related.")
        .author("Mohammed Al-Ani <mohammed.al-ani@infotamia.com>")
        .arg(Arg::with_name("stock")
            .short("s")
            .long("stock")
            .help("stock entry point")
            .requires("target")
            .required(true)
        )
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .takes_value(true)
            .help("stock target currency")
            .value_names(&allowed_values)
            .possible_values(&allowed_values)
            .allow_hyphen_values(true)
            .empty_values(false))
        .arg(Arg::with_name("infinite")
            .short("i")
            .long("infinite")
            .takes_value(true)
            .possible_values(&["true", "false"])
            .default_value("false")
            .required(false))
        .arg(Arg::with_name("email")
            .short("e")
            .long("email")
            .takes_value(true)
            .help("receipt email!")
            .empty_values(false)
            .required(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .allow_hyphen_values(true)
            .required(true)
            .empty_values(false)
            .takes_value(true)
            .help("email configuration path")
            .value_name("path/to/email_config.json")
            .default_value("email_config.json"))
        .arg(Arg::with_name("high")
            .short("hi")
            .long("high")
            .required(true)
            .takes_value(true)
            .empty_values(false)
            .validator(number_validator)
            .help("set the high threshold"))
        .arg(Arg::with_name("low")
            .short("lo")
            .long("low")
            .required(true)
            .takes_value(true)
            .empty_values(false)
            .validator(number_validator)
            .help("set the low threshold"))
        .get_matches_safe().unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    let symbol = matches.value_of("target").unwrap_or_else(|| {
        eprintln!("no command supplied");
        process::exit(1);
    });

    let email = matches.value_of("email").unwrap_or_else(|| {
        eprintln!("no email supplied");
        process::exit(1);
    });

    let low = matches.value_of("low").unwrap().parse::<i32>().unwrap();
    let high = matches.value_of("high").unwrap().parse::<i32>().unwrap();
    let name = format!("");

    let api = stock_api::YahooApi::new();
    use std::str::FromStr;
    let infinite: bool = FromStr::from_str(matches.value_of("infinite").unwrap_or_else(|| {
        eprintln!("no infinite command supplied");
        process::exit(1);
    })).unwrap_or(false);

    let email_config_path = matches.value_of("path").unwrap_or_else(|| {
        eprintln!("no infinite command supplied");
        process::exit(1);
    });



    let mut now = SystemTime::now();
    let email_service = email::EmailService::new(&email_config_path.to_string());
    let mut email_sender = |subject: String, content: String| {
        let seconds = now.elapsed().unwrap().as_secs();
        if seconds > 10 {
            println!("sending email!");
            email_service.send("info@infotamia.com", email, subject.as_str(), content.as_str());
            now = SystemTime::now();
        }
    };

    while infinite {
        thread::sleep(Duration::new(1,0));
        let option = api.fetch(&name, &symbol.to_string()).await;
        let stock_data = option.unwrap_or_else(|| {
            eprintln!("no stock data found!");
            process::exit(1);
        });

        let price = stock_data.regular_market_price.clone() as i32;
        if price >= high {
            email_sender(format!("Jackpot 3reeda, price =  {}", price), price.to_string());
        } else if price <= low {
            email_sender(format!("BTC is shit, price =  {}", price), stock_data.regular_market_price.to_string());
        }
        println!("{:?}", stock_data);
    }
    let option = api.fetch(&name, &symbol.to_string()).await;
    let stock_data = option.unwrap_or_else(|| {
        eprintln!("no stock data found!");
        process::exit(1);
    });
    println!("{:?}", stock_data);

}


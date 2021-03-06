#[macro_use] extern crate lazy_static;

mod config;
use config::*;

fn main() {
    let config = get_config();

    println!("Hello, world!");
    println!("App Name: {}", config.app_name);
    println!("App Version: {}", config.app_version);

    config::load_config_file();
    let config2 = get_config();

    println!("-------------------------------");
    println!("App Name: {}", config2.app_name);
    println!("App Version: {}", config2.app_version);

    let res = with_config(|config3| {
        println!("-------------------------------");
        println!("App Name: {}", config3.app_name);
        println!("App Version: {}", config3.app_version);

        with_config(|config4| {
            println!("-------------------------------");
            println!("App Name: {}", config4.app_name);
            println!("App Version: {}", config4.app_version);
            "res"
        })
    });

    println!("My result: {}", res);
}

#[macro_use] extern crate lazy_static;

mod config;

fn main() {
    let config = config::CONFIG.lock().unwrap().borrow().clone();

    println!("Hello, world!");
    println!("App Name: {}", config.app_name);
    println!("App Version: {}", config.app_version);

    config::load_config_file();
    let config2 = config::CONFIG.lock().unwrap().borrow().clone();

    println!("-------------------------------");
    println!("App Name: {}", config2.app_name);
    println!("App Version: {}", config2.app_version);
}

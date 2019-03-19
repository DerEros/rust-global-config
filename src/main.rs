#[macro_use] extern crate lazy_static;

mod config;

fn main() {
    let config = config::CONFIG.lock().unwrap().borrow().clone();

    println!("Hello, world!");
    println!("App Name: {}", config.app_name);
    println!("App Version: {}", config.app_version);
}

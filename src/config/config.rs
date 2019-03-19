use std::sync::Mutex;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub app_name: String,
    pub app_version: i32
}

lazy_static! {
    pub static ref CONFIG: Mutex<RefCell<Configuration>> = {
        Mutex::new(RefCell::new(get_default_config()))
    };
}

fn get_default_config() -> Configuration {
    Configuration {
        app_name: String::from("Default Appname"),
        app_version: 1
    }
}

pub fn load_config_file() {
    CONFIG.lock().unwrap().replace(load_config("App name from file", 2));
}

fn load_config(name: &str, version: i32) -> Configuration {
    Configuration {
        app_name: String::from(name),
        app_version: version
    }
}

pub fn get_config() -> Configuration {
    CONFIG.lock()
        .map(|config_ref|
            (*config_ref).borrow().clone()
        ).expect("Configuration missing completely!")
}
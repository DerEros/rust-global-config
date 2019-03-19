use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub app_name: String,
    pub app_version: i32
}

lazy_static! {
    pub static ref CONFIG: RwLock<Configuration> = {
        RwLock::new(get_default_config())
    };
}

fn get_default_config() -> Configuration {
    Configuration {
        app_name: String::from("Default Appname"),
        app_version: 1
    }
}

pub fn load_config_file() {
    *CONFIG.write().unwrap() = load_config("App name from file", 2);
}

fn load_config(name: &str, version: i32) -> Configuration {
    Configuration {
        app_name: String::from(name),
        app_version: version
    }
}

pub fn get_config() -> Configuration {
    CONFIG.read()
        .map(|config_ref|
            (*config_ref).clone()
        ).expect("Configuration missing completely!")
}

pub fn with_config<F, R>(func: F) -> R
    where F: FnOnce(&Configuration) -> R {

    let config: &Configuration = &CONFIG.read().unwrap(); // panic if config cannot be locked for read

    func(config)
}
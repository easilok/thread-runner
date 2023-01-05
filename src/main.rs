mod thread_executer;
mod config;

use crate::thread_executer::Executer;
use crate::config::Config;

fn main() {
    let config = Config::build("config.toml").unwrap();

    println!("{:?}", config);

    println!("Lets execute a new program here");

    let mut handles = vec![];

    // Mandatory to have the environment array defined into the config.toml file, even if empty
    for env in config.environment.unwrap().into_iter() {
        let mut handle = Executer::new(env, config.base_dir.to_string(), config.command.to_string());
        handle.start();
        handles.push(handle);
    }

    loop {}
}

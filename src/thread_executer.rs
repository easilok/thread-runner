use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

pub struct Executer {
    base_dir: String,
    command: String,
    environment: HashMap<String,String>,
    sender: Option<mpsc::Sender<bool>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Executer {
    pub fn new(environment: HashMap<String, String>, base_dir: String, command: String) -> Executer {
        Executer {
            sender: None,
            thread: None,
            base_dir,
            command,
            environment
        }
    }

    pub fn start(&mut self) {
        let (sender, receiver): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();

        self.sender = Some(sender);

        let base_dir = self.base_dir.clone();
        let command = self.command.clone();
        let environment = self.environment.clone();

        self.thread = Some(thread::spawn(move || {
            let env_log = environment.clone();
            let mut child = Command::new("sh")
                .current_dir(base_dir)
                // .env("EMULATOR_IMEI", &imei)
                .envs(environment)
                .arg("-c")
                .arg(command)
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .spawn()
                .expect("failed to execute process");

            println!("Started process with environment {:?}", env_log);

            receiver.recv().unwrap();

            println!("Will kill process with environment {:?}", env_log);
            child.kill().unwrap();
        }));
    }
}

impl Drop for Executer {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            if let Ok(_) = sender.send(true) {
                println!("Thread shutdown")
            } else {
                println!("Failed to shutdown thread")
            }
        }

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

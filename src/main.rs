pub mod gui;
use std::env;
use std::io::Write;
use std::process;
use std::process::Command;
use std::io;

fn input(prompt: &'static str, target: &mut String) {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(target)
        .expect("C: cannot read stdin");
}

#[derive(PartialEq)]
pub enum Options {
    Shutdown,
    Reboot,
    Sleep,
    Help,
}

impl Options {
    fn new(ent: String) -> Result<Options, &'static str> {
        match ent.trim().to_lowercase().as_str() {
            "--shutdown" | "-sd" => Ok(Options::Shutdown),
            "--reboot" | "-r" => Ok(Options::Reboot),
            "--sleep" | "-sl" => Ok(Options::Sleep),
            "--help" | "-h" => Ok(Options::Help),
            _ => Err("Invalid argument (enter --help or -h to read the handbook)")
        }
    }
    fn execute(&self) -> Result<(), &'static str> {
        let cmd = match self {
            Self::Shutdown => "poweroff",
            Self::Reboot => "reboot",
            Self::Sleep => "suspend",
            Self::Help => {
                println!("---- POWERCTL HANDBOOK ----");
                println!("{:<12} {:<6} {}", "Full", "Short", "Description");
                println!("{:<12} {:<6} {}", "--shutdown", "-sd", "Shut down the computer");
                println!("{:<12} {:<6} {}", "--reboot", "-r", "Reboot the computer");
                println!("{:<12} {:<6} {}", "--sleep", "-sl", "Put the computer to sleep");
                println!("{:<12} {:<6} {}", "--help", "-h", "read the powerctl handbook");
                return Ok(());
            }
        };
        let status = Command::new("systemctl").arg(cmd).status();
        match status {
            Ok(_) => return Ok(()),
            Err(_) => {
                return Err("Invalid argument");
            }
        }
    }
}

fn parse(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() != 2 {
        return Err("You must enter exactly one argument");
    }
    Ok(args[1].clone())
}

fn main() {
    let args = env::args().collect();
    match parse(args) {
        Ok(opt) => {
            let opt = match Options::new(opt) {
                Ok(opt) => opt,
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(2);
                }
            };
            if env::var("GUI").is_ok_and(|v| v == "1" || v == "true") {
                if let Err(e) = gui::run(opt) {
                    eprintln!("GUI failed to start: {e}");
                    std::process::exit(3);
                }
            } else {
                if opt != Options::Help {
                    loop {
                        let mut buffer = String::new();
                        input("Do you want to continue? ", &mut buffer);
                        let buffer = buffer.trim().to_lowercase();
                        if buffer == "no" || buffer == "n" {
                            println!("exitting!");
                            break;
                        } else if buffer == "yes" || buffer == "y" {
                            match opt.execute() {
                                Ok(unit) => unit,
                                Err(err) => {
                                    eprintln!("Error: {err}");
                                    process::exit(4)
                                }
                            };
                            break;
                        } else {
                            eprintln!("Invalid option");
                        }
                    }
                } else {
                    let _ = opt.execute();
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };
}

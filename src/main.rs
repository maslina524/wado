use std::{env, process::Command};

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let exec_name = match argv.get(1) {
        Some(a) => a,
        None => {
            eprintln!("Wado: no input arguments provided");
            std::process::exit(1);
        }
    };
    let exec_argv = &argv[2..];

    let status = Command::new(exec_name)
        .args(exec_argv)
        .status();

    match status {
        Ok(s) => {
            match s.code() {
                Some(c) => std::process::exit(c),
                None => eprintln!("Wado: program did not return an exit status code")
            }
        },
        Err(e) => eprintln!("Wado: failed to get program exit status code: {e}")
    }
    std::process::exit(0)
}

use clap::{Arg, ArgAction, Command};
use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cmd = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(Arg::new("path").action(ArgAction::Append))
        .arg(
            Arg::new("host")
                .long("host")
                .required(false)
                .help("Host or IP being probed."),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .required(false)
                .help("TCP port being probed."),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .short('t')
                .required(false)
                .default_value("10")
                .help("Timeout in seconds before quitting."),
        );

    let matches = cmd.get_matches();

    let host = matches.get_one::<String>("host").unwrap();

    let port = matches.get_one::<String>("port").unwrap();

    let timeout: u64 = matches
        .get_one::<String>("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let timeout = Duration::from_secs(timeout);

    let start = std::time::Instant::now();

    loop {
        if start.elapsed() > timeout {
            println!("process timed out after {:?}s", timeout.as_secs());
            exit(0)
        }

        match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(_) => {
                println!(
                    "{}:{} is available after {} seconds",
                    host,
                    port,
                    start.elapsed().as_secs()
                );

                exit(0);
            }
            Err(e) => {
                println!("Error: {}", e);
                sleep(Duration::from_secs(1));
            }
        }
    }
}

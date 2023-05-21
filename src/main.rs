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
                .required_unless_present("path")
                .conflicts_with("path")
                .help("Host or IP being probed."),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .required_unless_present("path")
                .conflicts_with("path")
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

    let (host, port) = if let Some(path) = matches.get_one::<String>("path") {
        let parts: Vec<&str> = path.split(':').collect();
        if parts.len() != 2 {
            println!("Invalid path format. Please provide Host and Port separated by a colon (e.g., localhost.com:80).");
            exit(1);
        }
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (
            matches.get_one::<String>("host").unwrap().clone(),
            matches.get_one::<String>("port").unwrap().clone(),
        )
    };

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

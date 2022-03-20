use error_chain::error_chain;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate url;

use std::env;
use std::time::SystemTime;
use std::time::Duration;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let selection = &args[1];
    let sys_time = SystemTime::now();

    if selection == "-d" {
        let site = &args[2];
        let wordlist = &args[3];
        println!("Finding directories!");
        if let Ok(lines) = read_lines(wordlist) {
            for line in lines {
                if let Ok(ip) = line {
                    let res = reqwest::get(site.to_owned() + &ip).await?;
                    if res.status() == 200 {
                        println!("{}", site.to_owned() + &ip);
                    }
                }
            }
        }
    }
    if selection == "-f" {
        let site = &args[2];
        let wordlist = &args[3];
        println!("Finding subdomains!");
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        if let Ok(lines) = read_lines(wordlist) {
            for line in lines {
                if let Ok(line) = line {
                    let site2 = str::replace(site, "FUZZ", &line);
                    let res = client
                        .get(&site2)
                        .timeout(Duration::from_secs(10))
                        .send();
                    let res = match res.await {
                        Ok(v) => v,
                        Err(_) => {
                            continue;
                        }
                    };
                    if res.status() == 200 {
                        println!("{}", site2.to_owned());
                    }
                }
            }
        }
    }
    if selection == "-vh" {
        let site = &args[2];
        let wordlist = &args[3];
        println!("Finding subdomains!");
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        if let Ok(lines) = read_lines(wordlist) {
            for line in lines {
                if let Ok(line) = line {
                    let site3 = str::replace(site, "FUZZ.", "");
                    let site2 = str::replace(site, "http://FUZZ", &line);
                    let res = client
                        .get(&site3)
                        .header("host", site2.clone())
                        .timeout(Duration::from_secs(10))
                        .send();
                    let res = match res.await {
                        Ok(v) => v,
                        Err(_) => {
                            continue;
                        }
                    };
                    if res.status() == 200 {
                        println!("{}", site2.to_owned());
                    }
                }
            }
        }
    }
    if selection == "-h" {
        help();
    }
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time);
    println!("{:?}", difference);
    Ok(())
}

fn help() {
    println!("Usage: ./RustBuster [Options] http://(FUZZ).website.com/ Wordlist");
    println!("Options:");
    println!("-h : Display this help message.");
    println!("-d : Directory fuzzing");
    println!("-f : Subdomain fuzzing");
    println!("-vh : Virtual Host Subdomain fuzzing");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


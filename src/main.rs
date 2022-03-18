use error_chain::error_chain;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate url;
use std::env;
use std::time::SystemTime;



error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let site = &args[1];
    let wordlist = &args[2];
    let sys_time = SystemTime::now();
    if let Ok(lines) = read_lines(wordlist) {
        for line in lines {
            if let Ok(ip) = line {
                let res = reqwest::get(site.to_owned()+&ip).await?;
                if res.status() == 200 {
                    println!("{}",site.to_owned()+&ip);
                }
            }
        }
        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(sys_time);
        println!("{:?}", difference);
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




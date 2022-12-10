use std::error::Error;

use reqwest;

fn main() -> Result<(), Box<dyn Error>>{
    let url = "http://www.rustinaction.com/";
    let response = reqwest::blocking::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}

#![allow(unused)]

mod error;
mod config;
mod types;
mod app;

use app::App;

fn main() -> error::Result<()>{
    println!("1");
    let app = App {};
    app.run()?;

    Ok(())
}

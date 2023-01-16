mod error;
mod config;
mod types;
mod app;

use app::App;

fn main() -> error::Result<()>{
    let app = App {};
    app.run()?;

    Ok(())
}
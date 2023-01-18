mod error;
mod types;

mod services;
mod view;
mod controller;
mod app;

use view::View;

fn main() {
    let view = View;
    view.run();
}

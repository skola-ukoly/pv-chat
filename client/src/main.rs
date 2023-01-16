mod error;
mod types;

mod model;
mod view;
mod presenter;

use view::View;


fn main() {
    let view = View {};
    view.run();
}

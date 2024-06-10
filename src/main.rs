#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use todo_tutorial::app::App;
// const _STYLE: &str = manganis::mg!(file("./public/tailwind.css"));



fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}








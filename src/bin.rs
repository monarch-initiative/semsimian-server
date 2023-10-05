// This is a simple web server that uses the semsimian crate to compare two sets of terms
// TODO:
// - Initialize RustSemsimian object once and pass it to the compare_termsets function
// - Implement Serialize for RustSemsimian structs so that we can return it from the compare_termsets function

use semsimian_server::rocket;

pub fn launch() {
    rocket();
}

fn main() {
    launch();
}
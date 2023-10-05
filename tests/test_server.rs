// import funcions from src/main.rs
use crate::main::say_hello;

// test say_hello function
#[test]
fn test_run() {
    assert_eq!(say_hello(), "Semsimian Server Online");
}

// test compare_termsets function

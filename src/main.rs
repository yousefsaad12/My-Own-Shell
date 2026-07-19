mod commands;
mod path_utils;
mod repl;
mod executor;   


#[allow(unused_imports)]
use std::io::{self, Write};


fn main() {
    // TODO: Uncomment the code below to pass the first stage
    
    repl::run();
}

use std::fs::File;
use std::io::Write;

use clap::Parser;

use crate::seed;

// Boilerplate code for `.c` files used by both readable/runnable code versions
struct Boilerplate {
    fd: File,
}

impl Boilerplate {
    fn new(fd: File) -> Self {
        Self { fd }
    }

    fn init(&mut self) {
        self.fd.write(b"
#include <stdio.h>
")
            .expect("Failed to write to file");
    }
}
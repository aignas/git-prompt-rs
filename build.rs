// Copyright (c) 2017 fd developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

extern crate clap;
extern crate version_check;

use clap::IntoApp;
use clap_generate::{
    generate_to,
    generators::{Bash, Fish, Zsh},
};
use std::fs;
use std::io::{self, Write};
use std::process::exit;

include!("src/app.rs");

fn main() {
    match version_check::is_min_version("1.31.0") {
        Some(true) => {}
        // rustc version too small or can't figure it out
        _ => {
            writeln!(&mut io::stderr(), "'git-prompt' requires rustc >= 1.31.0").unwrap();
            exit(1);
        }
    }

    let outdir = std::env::var_os("SHELL_COMPLETIONS_DIR")
        .or(std::env::var_os("OUT_DIR"))
        .unwrap();
    if let Err(why) = fs::create_dir_all(&outdir) {
        panic!("couldn't create completion file dir: {}", why);
    }

    println!("building shell completions");
    let mut app = Opts::into_app();
    let bin_name = "git-prompt";
    generate_to::<Bash, _, _>(&mut app, bin_name, &outdir);
    generate_to::<Fish, _, _>(&mut app, bin_name, &outdir);
    generate_to::<Zsh, _, _>(&mut app, bin_name, &outdir);
}

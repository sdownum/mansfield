/*
 * This file is part of the Mansfield project, http://github.com/sdownum/mansfield
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2018 Steven Downum
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

extern crate rustyline;

use rustyline::Editor;
use rustyline::error::ReadlineError;

static PROGRAM:    &str = "mansfield";
static VERSION:    &str = "0.0.1";
static BRANCH:     &str = "rustyline";
static ARCH:       &str = "Windows x64";

fn main() {
    if std::env::args().count() != 1 {
        usage();
        std::process::exit(1);
    }

    do_repl();
}

fn do_repl() {
    let mut rl = Editor::<()>::new();

    // Print REPL header
    println!("{} {}-{}; {} version", PROGRAM, VERSION, BRANCH, ARCH);
    println!("Use Ctrl-C or Crtl-D to exit.");
    
    
    // REPL loop
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("{}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("{}: ERROR - {:?}", PROGRAM, err);
                break
            }
        }
    }
}

fn usage() {
    println!("usage: {} [OPTIONS]\n\n", PROGRAM);
    println!(" OPTIONS: \n");
    println!("No options currently defined\n");
}

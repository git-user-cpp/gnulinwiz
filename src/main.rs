/*
 * gnulinwiz AKA GNU/Linux Config Wizard: The ultimate post-installation setup assistant for Linux,
 * streamlining your configuration process with ease and precision.
 * 
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
 
use clap::Parser;
use gnulinwiz::gnu_linux_default_setup;
use std::process;

/// Command-line arguments for the GNU/Linux Config Wizard.
#[derive(Parser, Debug)]
#[command(
     author = "Andrew Kushyk",
     version = "1.0.0",
     about = "A post-installation setup assistant for Linux, streamlining configuration with ease and precision.",
     long_about = None
 )]
struct Args {
    /// Allow execution with root privileges
    #[arg(long)]
    allow_root: bool,
}

fn main() {
    let args = Args::parse();

    match gnu_linux_default_setup(args.allow_root) {
        Ok(()) => {
            println!("Setup completed successfully!");
        }
        Err(e) => {
            eprintln!("Setup failed: {}", e);
            process::exit(1);
        }
    }
}

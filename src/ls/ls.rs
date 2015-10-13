#![crate_name = "ls"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Roman Gafiyatullin <r.gafiyatullin@me.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;

#[path="../common/util.rs"]
#[macro_use]
mod util;
mod ls_opts;
mod ls_config;

use std::io::Write;
use std::fmt::Debug;
use getopts::{Options, Matches, Fail};

static NAME: &'static str = "ls";
static VERSION: &'static str = "0.0.1";

pub fn uumain(args: Vec<String>) -> i32 {
	let (opts, maybe_matches) = ls_opts::do_getopts( &args );
	let matches = main_ensure_opt_matches_or_exit( maybe_matches );
	main_maybe_display_help_or_version_and_exit( &opts, &matches );

	let maybe_input_config = ls_config::input_config( &matches );
	let input_config = main_ensure_config_or_exit( maybe_input_config );
	debug_maybe_dump_config( "LS_DEBUG_INPUT_CONFIG", &input_config );

	let maybe_output_config = ls_config::output_config( &matches );
	let output_config = main_ensure_config_or_exit( maybe_output_config );
	debug_maybe_dump_config( "LS_DEBUG_OUTPUT_CONFIG", &output_config );


	// for file in matches.free { process_file( &input_config, &output_config, &file ) }

	crash!( 127, "Just a placeholder for now" );
}

fn main_ensure_opt_matches_or_exit( maybe_matches: Result<Matches, Fail> ) -> Matches {
	match maybe_matches {
		Ok( matches ) => matches,
		Err( getopts_fail ) => crash!( 2, "{:?}", getopts_fail ),
	}
}

fn main_maybe_display_help_or_version_and_exit( opts: &Options, matches: &Matches ) {
	if matches.opt_present( "help" ) {
		let brief = format!("Usage: {} [OPTIONS] [FILE1 [FILE2 [FILE3 ... ]]]\nList information about the FILEs (the current directory by default).\n", NAME);
    	print!("{}", opts.usage(&brief));
    	std::process::exit(0);
	}
	else if matches.opt_present( "version" ) {
		println!("{} {}", NAME, VERSION);
		std::process::exit(0);
	}
}

fn main_ensure_config_or_exit<T>( maybe_config: Result<T, String> ) -> T {
	match maybe_config {
		Ok( config ) => config,
		Err( reason ) => crash!( 2, "{}", reason ),
	}
}

fn debug_maybe_dump_config<T: Debug>( var_name: &str, config: &T ) {
    use std::env;

    if let Ok(debug_var) = env::var( var_name ) {
        if debug_var == "1" {
            println!("{}", var_name);
            println!("\t{:?}", config);
        }
    }
}

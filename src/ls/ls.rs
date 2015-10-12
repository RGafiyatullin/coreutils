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

use std::io::{Write};
use getopts::{Options, Matches, Fail};

static NAME: &'static str = "ls";
static VERSION: &'static str = "0.0.1";

pub fn uumain(args: Vec<String>) -> i32 {
	let (opts, maybe_matches) = ls_opts::do_getopts( &args );
	main_ensure_opt_matches_or_exit( &maybe_matches );
	let matches = maybe_matches.ok().unwrap();
	main_maybe_display_help_or_version_and_exit( &opts, &matches );

	crash!( 127, "Just a placeholder for now" );
}

fn main_ensure_opt_matches_or_exit( maybe_matches: &Result<Matches, Fail> ) {
	match maybe_matches {
		&Ok( _ ) => (),
		&Err( ref getopts_fail ) => crash!( 2, "{:?}", getopts_fail ),
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

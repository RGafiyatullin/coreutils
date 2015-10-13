/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Roman Gafiyatullin <r.gafiyatullin@me.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */


use getopts::Matches;

#[derive(Debug)]
pub struct InputConfig {
	pub filter_hide_dot_files: bool,
	pub filter_hide_parent_and_current: bool,
	pub filter_hide_tilda_files: bool,
}

impl InputConfig {
	pub fn new() -> Self {
		InputConfig {
			filter_hide_dot_files: true,
			filter_hide_parent_and_current: true,
			filter_hide_tilda_files: false,
		}
	}
}

#[derive(Debug)]
pub struct OutputConfig;

impl OutputConfig {
	pub fn new() -> Self {
		OutputConfig
	}
}

pub fn input_config( matches: &Matches ) -> Result<InputConfig, String> {
	let mut cfg = InputConfig::new();

	if matches.opt_present( "a" ) {
		cfg.filter_hide_dot_files = false;
		cfg.filter_hide_parent_and_current = false;
	}
	if matches.opt_present( "A" ) {
		cfg.filter_hide_parent_and_current = false;
	}
	if matches.opt_present( "B" ) {
		cfg.filter_hide_tilda_files = true;
	}

	Ok( cfg )
}
pub fn output_config( matches: &Matches ) -> Result<OutputConfig, String> {
	let mut cfg = OutputConfig::new();

	Ok( cfg )
}

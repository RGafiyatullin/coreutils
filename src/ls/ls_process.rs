/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Roman Gafiyatullin <r.gafiyatullin@me.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */


use ls_config::{InputConfig, OutputConfig};

pub fn process_file( input_config: &InputConfig, output_config: &OutputConfig, file: &String ) -> ( bool, bool ) {
	println!("# processing file '{}'", file);
	(false, false)
}

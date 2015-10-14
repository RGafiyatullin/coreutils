/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Roman Gafiyatullin <r.gafiyatullin@me.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */


use std::{fs,io,error};
use ls_config::{InputConfig, OutputConfig};

enum LsErrorReason {
	IO( io::Error ),
	Generic( String ),
}

pub struct LsError {
	major: bool,
	reason: LsErrorReason,
}
impl LsError {
	pub fn is_major( &self ) -> bool { self.major }
	pub fn description( &self ) -> String {
		match &self.reason {
			&LsErrorReason::IO( ref io_reason ) => { format!("{}", io_reason) },
			&LsErrorReason::Generic( ref description ) => { description.clone() },
		}
	}

	fn set_major( mut self, major: bool ) -> Self {
		self.major = major;
		self
	}

	fn from_err_io( io_reason: io::Error ) -> Self {
		LsError{ major: false, reason: LsErrorReason::IO( io_reason ) }
	}
	fn from_string( description: String ) -> Self {
		LsError{ major: false, reason: LsErrorReason::Generic( description ) }
	}
}

pub fn process_file( input_config: &InputConfig, output_config: &OutputConfig, file_path: &String ) -> Result<(), LsError> {
	let meta = try!( get_metadata( file_path ) );
	let file_type = meta.file_type();
	if file_type.is_dir() && !input_config.treat_directories_as_files { process_file_directory( input_config, output_config, file_path ) }
	else if file_type.is_dir() && input_config.treat_directories_as_files  { process_file_regular( input_config, output_config, file_path, &meta ) }
	else if file_type.is_file() { process_file_regular( input_config, output_config, file_path, &meta ) }
	else if file_type.is_symlink() { process_file_symlink( input_config, output_config, file_path, &meta ) }
	else { Err( LsError::from_string( format!("Unexpected file type") ).set_major( true ) ) }
}

fn process_file_directory( input_config: &InputConfig, output_config: &OutputConfig, file_path: &String ) -> Result<(), LsError> {
	Err( LsError::from_string( "not implemented 'process_file_directory'".to_string() ).set_major(false) )
}
fn process_file_symlink( input_config: &InputConfig, output_config: &OutputConfig, file_path: &String, meta: &fs::Metadata ) -> Result<(), LsError> {
	Err( LsError::from_string( "not implemented 'process_file_symlink'".to_string() ).set_major(false) )
}
fn process_file_regular( input_config: &InputConfig, output_config: &OutputConfig, file_path: &String, meta: &fs::Metadata ) -> Result<(), LsError> {
	Err( LsError::from_string( "not implemented 'process_file_regular'".to_string() ).set_major(false) )
}

fn get_metadata( file_path: &String ) -> Result< fs::Metadata, LsError > {
	match fs::metadata( file_path ) {
		Ok( meta ) => Ok( meta ),
		Err( reason ) => Err( LsError::from_err_io( reason ).set_major( true ) ),
	}
}

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

use std::io::{Write};
use getopts::{Options, Matches, Fail, ParsingStyle};

static NAME: &'static str = "ls";
static VERSION: &'static str = "0.0.1";

pub fn uumain(args: Vec<String>) -> i32 {
	let maybe_matches = do_getopts( &args );
	uumain_check_getopts( &maybe_matches );

	crash!( 127, "Just a placeholder for now" );
}

fn uumain_check_getopts( maybe_matches: &Result<Matches, Fail> ) {
	match maybe_matches {
		&Ok( _ ) => (),
		&Err( ref getopts_fail ) => crash!( 2, "{:?}", getopts_fail ),
	}
}

fn do_getopts( args: &Vec<String> ) -> Result<Matches, Fail> {
	let mut opts = Options::new();

	opts.parsing_style( ParsingStyle::StopAtFirstFree );

	opts.optflag("", "help", "display this help and exit");
	opts.optflag("", "version", "output version information and exit");

	opts.optflag("a", "all", "do not ignore entries starting with .");
	opts.optflag("A", "allmost-all", "do not list implied . and ..");
	opts.optflag("", "author", "with -l, print the author of each file");
	opts.optflag("b", "escape", "print C-style escapes for nongraphic characters");
	opts.optopt("", "block-size",
		"scale sizes by SIZE before printing them; e.g., '--block-size=M' prints sizes in units of 1,048,576 bytes; see SIZE format below", "SIZE");
	opts.optflag("B", "ignore-backups", "do not list implied entries ending with ~");
	opts.optflag("c", "",
		"with -lt: sort by, and show, ctime (time of last modification of file status information); \
with -l: show ctime and sort by name; otherwise: sort by ctime, newest first");
	opts.optflag("C", "", "list entries by columns");
	opts.optopt("", "color", "colorize the output; WHEN can be 'never', 'auto', or 'always' (the default); more info below", "WHEN");
	opts.optflag("d", "directory", "list directories themselves, not their contents");
	opts.optflag("D", "dired", "generate output designed for Emacs' dired mode");
	opts.optflag("f", "", "do not sort, enable -aU, disable -ls --color");
	opts.optflag("F", "classify", "append indicator (one of */=>@|) to entries");
	opts.optflag("", "file-type", "likewise (as '-F'/'--classify'), except do not append '*'");
	opts.optopt("","format", "across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C", "WORD" );
	opts.optflag("", "full-time", "like -l --time-style=full-iso");
	opts.optflag("g", "", " like -l, but do not list owner");
	opts.optflag("", "group-directories-first",
		"group directories before files;\n\n\
can be augmented with a --sort option, but any use of --sort=none (-U) disables grouping");
	opts.optflag("G", "no-group", "in a long listing, don't print group names");
	opts.optflag("h", "human-readable", "with -l, print sizes in human readable format (e.g., 1K 234M 2G)");
	opts.optflag("", "si", "likewise (as '-h'/'--human-readable'), but use powers of 1000 not 1024");
	opts.optflag("H", "dereference-command-line", "follow symbolic links listed on the command line");
	opts.optflag("", "dereference-command-line-symlink-to-dir", "follow each command line symbolic link\n\nthat points to a directory");
	opts.optopt("", "hide", "do not list implied entries matching shell PATTERN (overridden by -a or -A)", "PATTERN");
	opts.optopt("", "indicator-style", "append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)", "WORD");
	opts.optflag("i", "inode", "print the index number of each file");
	opts.optopt("I", "ignore", "do not list implied entries matching shell PATTERN", "PATTERN");
	opts.optflag("k", "kibibytes", "default to 1024-byte blocks for disk usage");
	opts.optflag("l", "", "use a long listing format");
	opts.optflag("L", "dereference", "when showing file information for a symbolic link, show information for the file the link references rather than for the link itself");
	opts.optflag("m", "", "fill width with a comma separated list of entries");
	opts.optflag("n", "numeric-uid-gid", "like -l, but list numeric user and group IDs");
	opts.optflag("N", "literal" ,"print raw entry names (don't treat e.g. control characters specially)");
	opts.optflag("o", "", "like -l, but do not list group information");
	opts.optopt("p", "indicator-style", "append / indicator to directories", "slash");
	opts.optflag("q", "hide-control-chars", "print ? instead of nongraphic characters");
	opts.optflag("", "show-control-chars", "show nongraphic characters as-is (the default, unless program is 'ls' and output is a terminal)");
	opts.optflag("Q", "quote-name", "enclose entry names in double quotes");
	opts.optopt("", "quoting-style", "use quoting style WORD for entry names: literal, locale, shell, shell-always, c, escape", "WORD");
	opts.optflag("r", "reverse", "reverse order while sorting");
	opts.optflag("R", "recursive", "list subdirectories recursively");
	opts.optflag("s", "size", "print the allocated size of each file, in blocks");
	opts.optflag("S", "", "sort by file size");
	opts.optopt("", "sort", "sort by WORD instead of name: none (-U), size (-S), time (-t), version (-v), extension (-X)", "WORD");
	opts.optflag("", "time",
		"with -l, show time as WORD instead of default modification time: atime or access or use (-u) ctime or status (-c); also use specified time as sort key if --sort=time");
	opts.optopt("", "time-style",
		"with  -l, show times using style STYLE: full-iso, long-iso, iso, locale, or +FORMAT; FORMAT is interpreted like in 'date'; \
if FORMAT is FORMAT1<newline>FORMAT2, then FORMAT1 applies to non-recent files and FORMAT2 to recent files; \
if STYLE is prefixed with 'posix-', STYLE takes effect only outside the POSIX locale", "WORD");
	opts.optflag("t", "", "sort by modification time, newest first");
	opts.optopt("T", "tabsize", "assume tab stops at each COLS instead of 8", "COLS");
	opts.optflag("u", "", "with -lt: sort by, and show, access time; with -l: show access time and sort by name; otherwise: sort by access time");
	opts.optflag("U", "", "do not sort; list entries in directory order");
	opts.optflag("v", "", "natural sort of (version) numbers within text");
	opts.optopt("w", "width", "assume screen width instead of current value", "COLS");
	opts.optflag("x", "", "list entries by lines instead of by columns");
	opts.optflag("X", "", "sort alphabetically by entry extension");
	opts.optflag("1", "", "list one file per line");


	opts.parse( &args[1..] )
}


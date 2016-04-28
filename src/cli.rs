//! Command argument types
//! The deserialized structures of the command args.

#![allow(non_camel_case_types,non_snake_case)]

use docopt;

docopt!(pub Args derive Debug,concat!("
Usage: tabbed_text [options]
	   tabbed_text --help

Formats a text file with the indentation (tabs) indicating sections and each line indicating a new entry

Options:
  -h, --help            Show this message
  --file                Make the prefix and suffix flags take filepaths instead of strings
  --prefix=STR          Output string prefix for the output [default: DEFAULT]
  --suffix=STR          Output string suffix for the output [default: ]
  --section-prefix=STR  Output string prefix for a section [default: <div>]
  --section-suffix=STR  Output string suffix for a section [default: </div>]
  --entry-prefix=STR    Output string prefix for a section [default: <p>]
  --entry-suffix=STR    Output string suffix for a section [default: </p>]
"),
	flag_file: bool,
	flag_prefix: String,
	flag_suffix: String,
	flag_section_prefix: String,
	flag_section_suffix: String,
	flag_entry_prefix: String,
	flag_entry_suffix: String,
);

///Workaround for the creation of Args because the docopt macro is not making everything public
#[inline(always)]
pub fn Args_docopt() -> docopt::Docopt{Args::docopt()}

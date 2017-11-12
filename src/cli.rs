//! Command argument types
//! The deserialized structures of the command args.

#[derive(StructOpt,Debug)]
#[structopt(name="tabbed_text" , about="Formats a text file with the indentation (tabs) indicating sections and each line indicating a new entry.\nReads from stdin and writes to stdout.")]
pub struct Args{
	#[structopt(long="file" , help="Make the prefix and suffix flags take filepaths instead of strings")]
	pub flag_file: bool,

	#[structopt(long="prefix" , help="Output string prefix for the output" , default_value="DEFAULT")]
	pub flag_prefix: String,

	#[structopt(long="suffix" , help="Output string suffix for the output" , default_value="")]
	pub flag_suffix: String,

	#[structopt(long="section-title-prefix" , help="Output string prefix for a section title" , default_value="<div>")]
	pub flag_section_title_prefix: String,

	#[structopt(long="section-prefix" , help="Output string prefix for a section" , default_value="<div>")]
	pub flag_section_prefix: String,

	#[structopt(long="section-suffix" , help="Output string suffix for a section" , default_value="</div>")]
	pub flag_section_suffix: String,

	#[structopt(long="entry-prefix" , help="Output string prefix for a section" , default_value="<p>")]
	pub flag_entry_prefix: String,

	#[structopt(long="entry-suffix" , help="Output string suffix for a section" , default_value="</p>")]
	pub flag_entry_suffix: String,
}

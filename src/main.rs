#![feature(io,plugin)]

extern crate structopt;
#[macro_use]extern crate structopt_derive;

mod cli;

use std::fs::File;
use std::io::{self,Read,Write};
use structopt::StructOpt;

//TODO: Implementing `flag_section_title_prefix` needs to buffer the previous line

fn main(){
	let mut args: cli::Args = cli::Args::from_args();
	let     input  = io::stdin();
	let mut output = io::stdout();

	if args.flag_file{
		//Prepare cached versions of the files
		if args.flag_entry_prefix         != ""{let mut file = File::open(&args.flag_entry_prefix).unwrap();        args.flag_entry_prefix.clear();        file.read_to_string(&mut args.flag_entry_prefix).unwrap();}
		if args.flag_entry_suffix         != ""{let mut file = File::open(&args.flag_entry_suffix).unwrap();        args.flag_entry_suffix.clear();        file.read_to_string(&mut args.flag_entry_suffix).unwrap();}
		if args.flag_section_title_prefix != ""{let mut file = File::open(&args.flag_section_title_prefix).unwrap();args.flag_section_title_prefix.clear();file.read_to_string(&mut args.flag_section_title_prefix).unwrap();}
		if args.flag_section_prefix       != ""{let mut file = File::open(&args.flag_section_prefix).unwrap();      args.flag_section_prefix.clear();      file.read_to_string(&mut args.flag_section_prefix).unwrap();}
		if args.flag_section_suffix       != ""{let mut file = File::open(&args.flag_section_suffix).unwrap();      args.flag_section_suffix.clear();      file.read_to_string(&mut args.flag_section_suffix).unwrap();}
	}

	//Prefix
	if args.flag_prefix=="DEFAULT"{
		write!(output,"{}{}{}{}",
			"<!DOCTYPE html>",
			"<meta charset=\"utf-8\">",
			"<html>",
			"<style>div>div{margin-left:2em;}div+div,div+p{margin-top:1em;}p{margin:0;margin-top:0.5em;font-family:\"Courier New\";font-size:13px;}div>p:first-child{margin-top:0.1em;}</style>"
		).unwrap();
	}else{
		if args.flag_file{
			io::copy(&mut File::open(args.flag_prefix).unwrap(),&mut output).unwrap();
		}else{
			write!(output,"{}",args.flag_prefix).unwrap();
		}
	}

	//Looping over lines
	let mut chars = input.chars();
	let mut tabs_current;
	let mut tabs_previous = 0;
	'Lines: loop{
		let mut c;
		tabs_current = 0;

		//Count tabs for the current line
		'Tabs: loop{match chars.next(){
			Some(Ok('\t')) => {
				tabs_current+= 1;
			},
			Some(Ok('\n')) => {
				write!(output,"{}{}",args.flag_section_suffix,args.flag_section_prefix).unwrap();
				continue 'Lines;
			},
			Some(chr) => {
				c = chr.unwrap();
				break 'Tabs;
			},
			None => {
				break 'Lines;
			}
		}}

		//Going deeper
		if tabs_current > tabs_previous{
			for _ in tabs_previous..tabs_current{
				write!(output,"{}",args.flag_section_prefix).unwrap();
			}
		}
		//Going shallower
		else if tabs_current < tabs_previous{
			for _ in tabs_current..tabs_previous{
				write!(output,"{}",args.flag_section_suffix).unwrap();
			}
		}

		//Entry prefix
		write!(output,"{}",args.flag_entry_prefix).unwrap();

		//Print characters on the current line (entry)
		'Line: loop{
			write!(output,"{}",c).unwrap();

			match chars.next(){
				Some(Ok('\n')) => {
					break 'Line;
				},
				Some(chr) => {
					c = chr.unwrap();
				},
				None => {
					break 'Lines;
				}
			}
		}
		//Entry suffix
		write!(output,"{}",args.flag_entry_suffix).unwrap();

		tabs_previous = tabs_current;
	}

	//Suffix
	if args.flag_file{
		io::copy(&mut File::open(args.flag_suffix).unwrap(),&mut output).unwrap();
	}else{
		write!(output,"{}",args.flag_suffix).unwrap();
	}
}


mod test{
	#[test]
	fn simple(){
		a
	}
}

use regex::{Regex, Match, Matches};
use std::iter::{Iterator, Peekable};
use std::cmp::PartialEq;

pub mod arg {
	pub struct Arg<'a>(&'a str, &'a str);

	impl<'a> PartialEq<str> for Arg<'a> {
		fn eq(&self, other: &str) -> bool {
			self.0 == other || self.1 == other
		}
	}

	pub static YEAR: Arg = Arg("-y", "--year");
	pub static LABEL: Arg = Arg("-l", "--label");
	pub static ALL: &[&str] = &[YEAR.0, YEAR.1, LABEL.0, LABEL.1];
}

mod re {
	use lazy_static::lazy_static;
	use regex::{Regex};

	lazy_static! {
		pub static ref DELIMITER: Regex = Regex::new(r"\s|\.|-").unwrap();
		pub static ref LABEL: Regex = Regex::new(r"[[:upper:]]{2,}").unwrap();
		pub static ref NUMBER: Regex = Regex::new(r"[[:digit:]]+").unwrap();
		pub static ref YEAR: Regex = Regex::new(r"(19|20)[\d]{2,2}").unwrap();
	}
}

#[derive(Clone, Copy)]
pub enum Parsed<'a> {
	Text(&'a str),
	Year(&'a str),
	Label(&'a str),
	Delimiter(&'a str),
	Arg(&'a str),
}

struct SetMatches<'t, 'a> {
	year: Peekable<Matches<'t, 'a>>,
	label: Peekable<Matches<'t, 'a>>,
	delimiter: Peekable<Matches<'t, 'a>>,
}

pub struct ReStrIterator<'t, 'a> {
	wrapped: &'a str,
	pos: usize,
	set_matches: SetMatches<'t, 'a>,
}

impl<'t, 'a> ReStrIterator<'t, 'a> {

	fn new(s: &'a str) -> ReStrIterator<'t, 'a> {
		ReStrIterator {
			wrapped: s,
			pos: 0,
			set_matches: SetMatches {
				year: re::NUMBER.find_iter(s).peekable(),
				label: re::LABEL.find_iter(s).peekable(),
				delimiter: re::DELIMITER.find_iter(s).peekable(),
			},
		}
	}

	fn is_closer_match(current: &mut Peekable<Matches<'t, 'a>>, candidate: &mut Peekable<Matches<'t, 'a>>) -> bool {
		match (current.peek(), candidate.peek()) {
			(_, None) => false,
			(None, _) => true,
			(Some(cur), Some(cand)) => cur.start() >= cand.start(),
		}
	}

	fn is_year(m: &Match) -> bool {
		re::YEAR.is_match(m.as_str()) && m.as_str().len() == 4
	}
}

impl<'t, 'a> Iterator for ReStrIterator<'t, 'a> {
	type Item = Parsed<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		use Parsed::*;

		if self.pos < self.wrapped.len() {

			let mut ret: Option<Parsed<'a>> = None;
			let mut next = &mut self.set_matches.year;

			if let Some(matched) = next.peek() {
				if ReStrIterator::is_year(matched) {
					ret = Some(Year(matched.as_str()));
				}
			}

			if ReStrIterator::is_closer_match(next, &mut self.set_matches.label) {
				next = &mut self.set_matches.label;
				ret = Some(Label(next.peek().unwrap().as_str()));
			}

			if ReStrIterator::is_closer_match(next, &mut self.set_matches.delimiter) {
				next = &mut self.set_matches.delimiter;
				ret = Some(Delimiter(next.peek().unwrap().as_str()));
			}

			if let Some(_) = &ret {
				let f_current_pos = next.peek().unwrap().start() == self.pos;

				if f_current_pos {  // Match is on the current position
					self.pos = next.next().unwrap().end();
				} else {  // Match is further than the current position, up until the string is interpreted as a text
					let npos = next.peek().unwrap().start();
					ret = Some(Text(&self.wrapped[self.pos..npos]));
					self.pos = npos;
				}
			} else {  // No matches ahead, the whole string is interpreted as a text
				ret = Some(Text(&self.wrapped[self.pos..]));
				self.pos = self.wrapped.len();
			}

			ret
		} else {
			None
		}
	}
}

pub fn test() {
	let s = "there2010.echoLABEL";
	// let s = "";

	for m in ReStrIterator::new(s) {
		match m {
			Parsed::Year(s) => println!("Year: {}", s),
			Parsed::Text(s) => println!("Generic text: {}", s),
			Parsed::Delimiter(s) => println!("Delimiter: {}", s),
			Parsed::Label(s) => println!("Label: {}", s),
			_ => {},
		}
	}
}

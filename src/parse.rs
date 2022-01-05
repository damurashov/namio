use std::vec::Vec;
use crate::args;
use regex::{Regex, Match};
use env;
use std::iter::{Iterator, IntoIterator};

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

enum Input<'a> {
	Word(&'a str),
	Year(&'a str),
	Label(&'a str)
}

struct ReStrIterator<'a> {
	wrapped: &'a str,
	pos: usize,
}

impl<'a> ReStrIterator<'a> {

	fn new(s: &'a str) -> ReStrIterator<'a> {
		ReStrIterator {wrapped: s, pos: 0}
	}

	fn parse_date(&mut self) -> Option<Match<'a>> {
		const YEAR_LEN: usize = 4;

		if let Some(mat) = re::NUMBER.find(&self.wrapped[self.pos..]) {
			if mat.end() - mat.start() == YEAR_LEN {
				return re::YEAR.find(&self.wrapped[self.pos..])
			}
		}

		None
	}

	fn parse_label(&mut self) -> Option<Match<'a>> {
		re::LABEL.find(&self.wrapped[self.pos..])
	}

	fn parse_delimiter(&mut self) -> Option<Match<'a>> {
		re::DELIMITER.find(&self.wrapped[self.pos..])
	}
}

impl<'a> Iterator for ReStrIterator<'a> {
	type Item = Input<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

struct StrWrapped<'a>(&'a str);  // Regex-iterable string

impl<'a> IntoIterator for StrWrapped<'a> {
	type Item = <ReStrIterator<'a> as Iterator>::Item;
	type IntoIter = ReStrIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(&self.0)
	}
}

fn split_extract_regex<'a>(s: &'a str, regex: &Regex) -> Vec<&'a str> {
	let mut res: Vec<&str> = Vec::new();
	let mut prev_end = 0;

	for m in regex.find_iter(s) {
		if m.start() != prev_end {
			res.push(&s[prev_end..m.start()]);
		}

		res.push(&s[m.start()..m.end()]);
		prev_end = m.end();
	}

	if prev_end != s.len() {
		res.push(&s[prev_end..]);
	}

	res
}

pub fn test() {
	let s = "there2010";
	let s_split = split_extract_regex(s, &re::YEAR);

	for val in s_split {
		println!("{}", val);
	}
}
use core::cmp::PartialEq;

pub struct Args<'a>(&'a str, &'a str, &'a str, &'a str);

impl<'a> Args<'a> {
	fn as_long_primary(&'a self) -> & str {
		&self.0
	}

	fn as_short_primary(&self) -> &str {
		&self.1
	}

	fn as_long_modified(&self) -> &str {
		&self.2
	}

	fn as_short_modified(&self) -> &str {
		&self.3
	}

	fn is_modified(&self, s: &str) -> bool {
		self.2 == s || self.3 == s
	}
}

impl<'a> PartialEq for Args<'a> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
	}
}

impl<'a> PartialEq<&str> for Args<'a> {
	fn eq(&self, other: &&str) -> bool {
		&self.0 == other || &self.1 == other || &self.2 == other || &self.3 == other
	}
}

pub const YEAR: &'static Args = &Args("--year", "-y", "--Year", "-Y");
pub const LABEL: &'static Args = &Args("--label", "-l", "--Label", "-L");
pub const DATE: &'static Args = &Args("--date", "-d", "", "");

pub const ALL: [&Args; 3] = [&YEAR, &LABEL, &DATE];

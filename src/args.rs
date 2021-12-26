use std::{borrow::Cow};

struct Argpair(Cow<'static, str>, Cow<'static, str>);

impl Argpair {
	fn long(&self) -> &str {
		&self.0
	}

	fn short(&self) -> &str {
		&self.1
	}
}

const APPEND_LABEL: Argpair = Argpair(Cow::Borrowed("--label"), Cow::Borrowed("-l"));
const SET_LABEL: Argpair = Argpair(Cow::Borrowed("--Label"), Cow::Borrowed("-L"));
const APPEND_YEAR: Argpair = Argpair(Cow::Borrowed("--year"), Cow::Borrowed("-y"));
const SET_YEAR: Argpair = Argpair(Cow::Borrowed("--Year"), Cow::Borrowed("-Y"));

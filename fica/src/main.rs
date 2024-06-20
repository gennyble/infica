use infica::Date;
use time::{format_description::FormatItem, macros::format_description};

const GREG: &[FormatItem] =
	format_description!("[weekday repr:long], [month repr:long] [day padding:none]");

fn main() {
	let now = Date::now_local().unwrap();

	println!("==TODAY==");
	print_date(now);

	if std::env::args().any(|s| s == "start") {
		let mut start = now;
		start.day = 1;

		println!("\n==START==");
		print_date(start);
	}

	if std::env::args().any(|s| s == "middle" || s == "mid") {
		let mut start = now;
		start.day = 15;

		println!("\n==MIDDLE==");
		print_date(start);
	}

	if std::env::args().any(|s| s == "end") {
		let mut start = now;
		start.day = 28;

		println!("\n==END==");
		print_date(start);
	}
}

fn print_date(date: Date) {
	let now = date; // lol
	println!(
		"{}, {} {}{} {}",
		now.day_name(),
		now.month_name(),
		now.day,
		ord(now.day as usize),
		now.year
	);

	if std::env::args().any(|s| s == "greg") {
		let greg = time::Date::from(now);
		let str = greg.format(GREG).unwrap();
		println!("{str}{} {}", ord(greg.day() as usize), greg.year());
	}
}

fn ord(num: usize) -> &'static str {
	if (10..=19).contains(&num) {
		// i'd like to make this fucked up math + min/max in the index if i can
		// but that is apparently difficult
		"th"
	} else {
		// teens - ordinals
		["th", "th", "st", "nd", "rd", "th"][(num % 10).min(4) + 1]
	}
}

#[cfg(test)]
mod test {
	use crate::ord;

	#[test]
	fn weird_ord_single() {
		assert_eq!(ord(0), "th");
		assert_eq!(ord(1), "st");
		assert_eq!(ord(2), "nd");
		assert_eq!(ord(3), "rd");
		assert_eq!(ord(4), "th");
		assert_eq!(ord(5), "th");
		assert_eq!(ord(6), "th");
		assert_eq!(ord(7), "th");
		assert_eq!(ord(8), "th");
		assert_eq!(ord(9), "th");
	}

	#[test]
	fn weird_ord_teens() {
		assert_eq!(ord(10), "th");
		assert_eq!(ord(11), "th");
		assert_eq!(ord(12), "th");
		assert_eq!(ord(13), "th");
		assert_eq!(ord(14), "th");
		assert_eq!(ord(15), "th");
		assert_eq!(ord(16), "th");
		assert_eq!(ord(17), "th");
		assert_eq!(ord(18), "th");
		assert_eq!(ord(19), "th");
	}

	#[test]
	fn weird_ord_etc() {
		assert_eq!(ord(20), "th");
		assert_eq!(ord(21), "st");
		assert_eq!(ord(42), "nd");
		assert_eq!(ord(5815), "th");
		assert_eq!(ord(2453), "rd");
		assert_eq!(ord(2341), "st");
	}

	#[test]
	fn weird_ord_high_teens() {
		assert_eq!(ord(111), "th", "should be one hundred and eleventh");
		assert_eq!(ord(112), "th");
		assert_eq!(ord(113), "th");
		assert_eq!(ord(114), "th");
		assert_eq!(ord(115), "th");
		assert_eq!(ord(116), "th");
		assert_eq!(ord(117), "th");
		assert_eq!(ord(118), "th");
		assert_eq!(ord(119), "th");
	}
}

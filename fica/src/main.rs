use infica::Date;
use owo_colors::OwoColorize;
use time::{format_description::FormatItem, macros::format_description};

const GREG: &[FormatItem] =
	format_description!("[weekday repr:long], [month repr:long] [day padding:none]");

const GREG_SMALL8601: &[FormatItem] = format_description!("[month repr:long] [day padding:none]");

fn main() {
	let now = Date::now_local().unwrap();
	let arg = std::env::args().nth(1);

	match arg.as_deref() {
		Some("elapsed") => {
			print_elapsed_weeks(now);
		}
		Some("month") | Some("current") => {
			print_month_boundaries(now);
		}
		Some(mo) => match infica::month_index(mo) {
			None => {
				println!("{} is not a recognized IFC month", mo.cyan());
			}
			Some(idx) => {
				let mut month_date = now;
				month_date.month = idx;

				print_month_boundaries(month_date);
			}
		},
		None => println!(
			"Today is {} in the IFC\nIt is week {}",
			now.rfc2822().cyan(),
			now.week().cyan()
		),
	}
}

fn print_elapsed_weeks(now: Date) {
	let current_week = now.week();

	for week in 1..=current_week {
		let start_day = (week - 1) % 4 * 7 + 1;
		let start = format_greg8601(Date {
			year: now.year,
			month: 1 + week / 4,
			day: start_day,
		});

		let end = format_greg8601(Date {
			year: now.year,
			month: 1 + week / 4,
			day: start_day + 7,
		});

		if week % 2 == 1 {
			println!(
				"Week {} started on {} and ended on {}",
				week.cyan(),
				start.cyan(),
				end.cyan()
			)
		} else {
			println!(
				"Week {} started on {} and ended on {}",
				week.yellow(),
				start.yellow(),
				end.yellow()
			)
		};
	}
}

fn print_month_boundaries(ifc: Date) {
	let mut start = ifc;
	start.day = 1;

	let mut mid = ifc;
	mid.day = 15;

	let mut end = ifc;
	end.day = 28;

	println!(
		"The IFC month of {} has Gregorian boundaries of {}\nand {} with a midpoint of {}",
		start.month_name().cyan(),
		format_greg(start).green(),
		format_greg(end).red(),
		format_greg(mid).yellow()
	);
}

fn format_greg<G: Into<time::Date>>(date: G) -> String {
	let greg = date.into();
	let str = greg.format(GREG).unwrap();
	format!("{str}{} {}", ord(greg.day() as usize), greg.year())
}

fn format_greg8601<G: Into<time::Date>>(date: G) -> String {
	let greg = date.into();
	let str = greg.format(GREG_SMALL8601).unwrap();
	format!("{str}{}", ord(greg.day() as usize))
}

fn ord(num: usize) -> &'static str {
	let num = num % 100; //ordinals repeat every 100

	if (10..=19).contains(&num) {
		"th"
	} else {
		["th", "st", "nd", "rd", "th"][(num % 10).min(4)]
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

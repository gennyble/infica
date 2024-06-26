//! The international fixed calendar is a 13-month calendar with each month
//! containing exactly 28 days. There is an extra day at the end of the year
//! called the year day.
//!
//! In leap-years there is an extra day inserted at the end of June called the
//! leap day. It is directly after the fourth week of june and is given to june,
//! so it becomes June 29th. The day after June 29th starts the new month, Sol,
//! with Sol 1.
//!
//! [Wikipedia: International Fixed Calendar][wp-ifc]
//! [wp-ifc]: https://en.wikipedia.org/wiki/International_Fixed_Calendar

#[cfg(feature = "passtime")]
pub use time;

use time::OffsetDateTime;

/// A day in the International Fixed Calendar. See crate level documentation for
/// more information about this calendar system.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Date {
	pub year: u32,
	/// The month of the year (1 indexed)
	pub month: u8,
	/// The day of the month (1 indexed)
	pub day: u8,
}

impl Date {
	/// Get the current date in the UTC timezone
	pub fn now_utc() -> Self {
		let utc = OffsetDateTime::now_utc();
		Date::from(utc.date())
	}

	/// Get the current date in your local timezone. or return an Error if we
	/// can't get it.
	#[cfg(feature = "local-offset")]
	pub fn now_local() -> Result<Self, time::error::IndeterminateOffset> {
		OffsetDateTime::now_local().map(|odt| Date::from(odt.date()))
	}

	/// Is this year a leap year?
	pub fn is_leap(&self) -> bool {
		year_leaps(self.year)
	}

	/// The number of days elapsed in the current year (1 indexed)
	pub fn ordinal(&self) -> u16 {
		let ord = (self.month as u16 - 1) * 28 + self.day as u16;

		//if leap year and the day is on/after leap day and it isn't leap day
		if self.is_leap() && ord >= 169 && !(self.month == 6 && self.day == 29) {
			ord + 1
		} else {
			ord
		}
	}

	/// The full name of the day. Sunday, Monday, etc.
	pub fn day_name(&self) -> &'static str {
		match (self.day - 1) % 7 {
			0 => "Sunday",
			1 => "Monday",
			2 => "Tuesday",
			3 => "Wednesday",
			4 => "Thursday",
			5 => "Friday",
			6 => "Saturday",
			_ => unreachable!(),
		}
	}

	/// The short, 3-letter name of the day. Sun, Mon, etc.
	pub fn day_name_short(&self) -> &'static str {
		match (self.day - 1) % 7 {
			0 => "Sun",
			1 => "Mon",
			2 => "Tue",
			3 => "Wed",
			4 => "Thu",
			5 => "Fri",
			6 => "Sat",
			_ => unreachable!(),
		}
	}

	/// The full name of the month. June, Sol, July, etc.
	pub fn month_name(&self) -> &'static str {
		MONTHS[self.month as usize - 1][0]
	}

	/// The short, 3 letter, name of the month. Jun, Sol, Jul, etc.
	pub fn month_name_short(&self) -> &'static str {
		MONTHS[self.month as usize - 1][1]
	}

	/// The current date formatted to mimick the format from RFC 2822
	pub fn rfc2822(&self) -> String {
		format!(
			"{}, {} {} {}",
			self.day_name_short(),
			self.day,
			self.month_name(),
			self.year
		)
	}
}

impl From<time::Date> for Date {
	fn from(date: time::Date) -> Self {
		let year = date.year() as u32;
		let ord = date.ordinal() - 1;
		let leap = year_leaps(year);

		if leap && ord == 168 {
			// Catch the leap day
			return Self {
				year,
				month: 6,
				day: 29,
			};
		} else if (!leap && ord == 364) || ord == 365 {
			// Catch both year days
			return Self {
				year,
				month: 13,
				day: 29,
			};
		}

		if !leap || ord <= 168 {
			// not a leap year path
			// also the "leap year but before the leap-day" path
			Self {
				year,
				month: (ord / 28) as u8 + 1,
				day: (ord % 28) as u8 + 1,
			}
		} else {
			Self {
				year,
				month: ((ord - 1) / 28) as u8 + 1,
				day: ((ord - 1) % 28) as u8 + 1,
			}
		}
	}
}

impl From<Date> for time::Date {
	fn from(date: Date) -> Self {
		time::Date::from_ordinal_date(date.year as i32, date.ordinal()).unwrap()
	}
}

const MONTHS: [[&str; 2]; 13] = [
	["January", "Jan"],
	["February", "Feb"],
	["March", "Mar"],
	["April", "Apr"],
	["May", "May"],
	["June", "Jun"],
	["Sol", "Sol"],
	["July", "Jul"],
	["August", "Aug"],
	["September", "Sep"],
	["October", "Oct"],
	["November", "Nov"],
	["December", "Dec"],
];

/// Whether or not a year is a leap year
fn year_leaps(year: u32) -> bool {
	let four = year % 4 == 0;
	let hundreds = year % 100 == 0;
	let fourhund = year % 400 == 0;

	// leap if:
	// - four AND NOT hundred
	// - four AND hundred AND fourhund

	// `fourhund` here checks `hundreds` by virtue of 100 being a multiple of 400
	four && (!hundreds || fourhund)
}

#[cfg(test)]
mod test {
	use time::Duration;

	use crate::year_leaps;

	#[test]
	fn leap_years() {
		// the examples given by wikipedia
		assert!(year_leaps(2000));
		assert!(!year_leaps(1700));
		assert!(!year_leaps(1800));
		assert!(!year_leaps(1900));

		// testing the four rule
		assert!(year_leaps(2024));
	}

	#[allow(clippy::zero_prefixed_literal)]
	#[test]
	fn correctly_upconverts_boundaries() {
		macro_rules! check {
			($gyear:literal - $gmonth:literal - $gday:literal, $year:literal - $month:literal - $day:literal) => {{
				let exp = crate::Date::from(
					time::Date::from_calendar_date(
						$gyear,
						time::Month::try_from($gmonth).unwrap(),
						$gday,
					)
					.unwrap(),
				);

				let act = crate::Date {
					year: $gyear,
					month: $month,
					day: $day,
				};

				if exp != act {
					if $gyear == $year {
						panic!(
							"Failed on, year {}, {}-{} // IFC {}-{}. Greg converted to {}-{}",
							$gyear, $gmonth, $gday, $month, $day, exp.month, exp.day
						)
					} else {
						panic!(
							"Failed on, {}-{}-{} // IFC {}-{}-{}. Greg converted to {}-{}-{}",
							$gyear,
							$gmonth,
							$gday,
							$year,
							$month,
							$day,
							exp.year,
							exp.month,
							exp.day
						)
					}
				}
			}};

			// The year in here has to be a non-leap year
			($gmonth:literal - $gday:literal, $month:literal - $day:literal) => {
				check!(2023 - $gmonth - $gday, 2023 - $month - $day)
			};

			// The year in here has to be a leap year
			(leap $gmonth:literal - $gday:literal, $month:literal - $day:literal) => {
				check!(2024 - $gmonth - $gday, 2024 - $month - $day)
			};
		}

		// January
		check!(01 - 01, 01 - 01);
		check!(01 - 28, 01 - 28);

		// February
		check!(01 - 29, 02 - 01);
		check!(02 - 25, 02 - 28);

		// March
		check!(02 - 26, 03 - 01);
		check!(03 - 25, 03 - 28);

		check!(leap 03 - 24, 03 - 28);

		// April
		check!(03 - 26, 04 - 01);
		check!(04 - 22, 04 - 28);

		check!(leap 03 - 25, 04 - 01);
		check!(leap 04 - 21, 04 - 28);

		// May
		check!(04 - 23, 05 - 01);
		check!(05 - 20, 05 - 28);

		check!(leap 04 - 22, 05 - 01);
		check!(leap 05 - 19, 05 - 28);

		// June
		check!(05 - 21, 06 - 01);
		check!(06 - 17, 06 - 28);

		check!(leap 05 - 20, 06 - 01);
		check!(leap 06 - 16, 06 - 28);

		// Leap day
		check!(leap 06 - 17, 06 - 29);

		// Sol
		check!(06 - 18, 07 - 01);
		check!(07 - 15, 07 - 28);
		//leap years don't change conversion after leap day
		check!(leap 06 - 18, 07 - 01);
		check!(leap 07 - 15, 07 - 28);

		// July
		check!(07 - 16, 08 - 01);
		check!(08 - 12, 08 - 28);

		// August
		check!(08 - 13, 09 - 01);
		check!(09 - 09, 09 - 28);

		// September
		check!(09 - 10, 10 - 01);
		check!(10 - 07, 10 - 28);

		// October
		check!(10 - 08, 11 - 01);
		check!(11 - 04, 11 - 28);

		// November
		check!(11 - 05, 12 - 01);
		check!(12 - 02, 12 - 28);

		// December
		check!(12 - 03, 13 - 01);
		check!(12 - 30, 13 - 28);
		check!(leap 12 - 30, 13 - 28);

		// Year Day
		check!(12 - 31, 13 - 29);
		check!(leap 12 - 31, 13 - 29);
	}

	#[test]
	fn round_trip_known_culprits() -> Result<(), time::error::ComponentRange> {
		let mut ifc: crate::Date;
		let june_17th_2024 = time::Date::from_calendar_date(2024, time::Month::June, 17)?;
		ifc = june_17th_2024.into();
		assert_eq!(
			june_17th_2024,
			ifc.into(),
			"ifc IR was {ifc:?}, ordinal: {}",
			ifc.ordinal()
		);

		let december_30th_2024 = time::Date::from_calendar_date(2024, time::Month::December, 30)?;
		ifc = december_30th_2024.into();
		assert_eq!(december_30th_2024, ifc.into(), "ifc IR was {ifc:?}");
		Ok(())
	}

	#[test]
	fn round_trip() {
		let today_greg = time::OffsetDateTime::now_utc().date();
		let today_ifc: crate::Date = today_greg.into();
		assert_eq!(today_greg, today_ifc.into());

		//check 20 years in the past and future
		let broken_dates: Vec<_> = (0..(366 * 20))
			.map(|days| {
				(
					today_greg + Duration::days(days),
					today_greg - Duration::days(days),
				)
			})
			.flat_map(|(tomorrow_greg, yesterday_greg)| {
				let mut retval = vec![];
				let mut new_ifc: crate::Date = tomorrow_greg.into();
				if tomorrow_greg != new_ifc.into() {
					retval.push((tomorrow_greg, new_ifc));
				}

				new_ifc = yesterday_greg.into();
				if yesterday_greg != new_ifc.into() {
					retval.push((yesterday_greg, new_ifc));
				}

				retval
			})
			.take(10)
			.collect();
		assert!(broken_dates.is_empty(), "{broken_dates:?}")
	}

	//TODO: gen- test IFC -> greg

	//TODO: gen- test IFC ordinal correct
}

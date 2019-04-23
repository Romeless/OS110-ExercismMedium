use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
	hours: i32,
	minutes: i32,
}

impl Clock {

    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
		while minutes >= 60 || minutes < 0 || hours < 0 {
			if minutes >= 60 {
				hours = hours + (minutes / 60) as i32;
				minutes = minutes % 60;
			}
			
			if minutes < 0 {
				hours = hours - 1 + (minutes / 60) as i32;
				minutes = 60 + minutes % 60;
			}
			
			if hours < 0 {
				hours = 24 + (hours % 24) as i32;
			}
		}
		
		hours = hours % 24;
		minutes = minutes % 60;
		Clock { hours: hours, minutes: minutes }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
		let mut new_hours : i32 = self.hours;
		
		if minutes >= 0 {
			let new_minutes = (self.minutes + minutes) % 60;
			new_hours = (self.hours + ((self.minutes + minutes) / 60 ) as i32) % 24;
			self.hours = new_hours;
			self.minutes = new_minutes;
			
			self
		} else {
			let mut new_minutes = self.minutes + minutes;
			
			if new_minutes < 0 {
				new_hours = self.hours - 1 + new_minutes / 60 as i32;
				new_minutes = 60 + new_minutes % 60;
			}
			
			if new_hours < 0 {
				new_hours = 24 + (new_hours % 24);
			}
			
			if new_hours >= 24 {
				new_hours = new_hours % 24
			}
			
			self.hours = new_hours;
			self.minutes = new_minutes;
			
			self
			
		}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let hh : String = if self.hours >= 10 { format!("{}", self.hours) } else { format!("0{}", self.hours) };
		let mm : String = if self.minutes >= 10 { format!("{}", self.minutes) } else { format!("0{}", self.minutes) };
		
        write!(f, "{}:{}", hh, mm)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hh : String = if self.hours >= 10 { format!("{}", self.hours) } else { format!("0{}", self.hours) };
		let mm : String = if self.minutes >= 10 { format!("{}", self.minutes) } else { format!("0{}", self.minutes) };
		
        write!(f, "{}:{}", hh, mm)
    }
}

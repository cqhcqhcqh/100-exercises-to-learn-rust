// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    // TODO
    temperatures: [Option<i32>; 7]
}
#[derive(Clone, Copy)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl Weekday {
    fn value(&self) -> usize {
        return *self as usize;
    }
}

impl WeekTemperatures {
    pub fn new() -> Self {
        // todo!()
        let temperates: [Option<i32>; 7] = [None; 7];
        return WeekTemperatures {temperatures: temperates}
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        // todo!()
        // match day {
        //     Weekday::Monday => return self.temperatures.get(0),
        //     Weekday::Tuesday => return self.temperatures.get(1),
        //     Weekday::Wednesday => return self.temperatures.get(2),
        //     Weekday::Thursday => return self.temperatures.get(3),
        //     Weekday::Friday => return self.temperatures.get(4),
        //     Weekday::Saturday => return self.temperatures.get(5),
        //     Weekday::Sunday => return self.temperatures.get(6),
        // }
        // let res = self.temperatures.get(day.value());
        // match res {
        //     Option::Some(v) => return Some(v?),
        //     Option::None => return None
        // }
        self.temperatures[day.value()]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        // todo!()
        // match day {
        //     Weekday::Monday => return self.temperatures[0] = temperature,
        //     Weekday::Tuesday => return self.temperatures[1] = temperature,
        //     Weekday::Wednesday => return self.temperature[2] = temperature,
        //     Weekday::Thursday => return self.temperatures[3] = temperature,
        //     Weekday::Friday => return self.temperatures[4] = temperature,
        //     Weekday::Saturday => return self.temperatures[5] = temperature,
        //     Weekday::Sunday => return self.temperatures[6] = temperature,
        // }
        self.temperatures[day.value()] = Some(temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}

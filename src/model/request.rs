//Copyright 2019 Anthony Bocci
//This file is part of Temperature.
// Temperature is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.

// Temperature is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Temperature.  If not, see <https://www.gnu.org/licenses/>.

/*!
 * Module request.
 *
 * Here are defined the request models and methods.
 */

use crate::http::JsonSerializable;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use super::Temperature;

/**
 * Represents a measured temperature that will be exported.
 */
pub struct Measure {
    /// The measured temperature.
    pub temperature: Temperature,
    /// The timestamp when the temperature was measured.
    pub timestamp: u64,
}

impl Measure {
    /// Create a new Measure.
    pub fn new(temperature: Temperature, timestamp: u64) -> Measure {
        Measure { temperature, timestamp }
    }

    /// Set the timestamp to now.
    pub fn init_timestamp(&mut self) {
        self.timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Unable to read the current timestamp.")
        .as_secs();
    }
}

impl JsonSerializable for Measure {
    /// Json serialize the measure to be sendable via HTTP.
    fn json_serialize(&self) -> HashMap<&str, String> {
        let mut map: HashMap<&str, String> = HashMap::new();
        match self.temperature {
            Temperature::Celsius(t) => {
                map.insert("temperature", t.to_string());
                map.insert("temperature_unit", String::from("celsius"));
            },
            Temperature::Fahrenheit(t) => {
                map.insert("temperature", t.to_string());
                map.insert("temperature_unit", String::from("fahrenheit"));
            }
        }
        map.insert("measured_time", self.timestamp.to_string());
        map
    }
}
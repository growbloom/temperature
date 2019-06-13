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
 * Module ds18b20.
 *
 * Here are defined the Ds18b20 struct and its methods.
 */

use crate::model::Temperature;
use std::fs;

/// The temperature sensor DS18B20
pub struct Ds18b20<'a> {
    /// The path to the item to read.
    pub path: &'a str,
}

impl<'a> crate::sensor::Sensor for Ds18b20<'a> {
    /// The name of the sensor.
    const NAME: &'static str = "DS18B20";

    /// Reads the temperature
    /// The temperature is read from the `path` that should be set before
    /// calling the read() method.
    fn read(&self) -> Temperature {
        let temperature: f64 = fs::read_to_string(&self.path)
        .expect("Unable to read the given path.")
        .lines()
        .nth(1)
        .expect("The data read is invalid.")[29..]
        .parse()
        .expect("The data read is invalid.");
        Temperature::Celsius(temperature / 1000.0)
    }

    fn new() -> Self {
        Ds18b20 { path: "" }
    }
}

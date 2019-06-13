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
 * Module sensor.
 *
 * Here is defined the Sensor enum.
 */

use crate::model::Temperature;

pub trait Sensor {
    /// The name of the sensor.
    const NAME: &'static str;

    /// Reads the temperature.
    fn read(&self) -> Temperature;

    // Create a new sensor.
    fn new() -> Self;
}

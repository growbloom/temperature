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
 * Module temperature.
 *
 * Here is defined the Temperature enum.
 */

extern crate clap;

use clap::{App, Arg};
use temperature::sensor;
use temperature::sensor::Sensor;

fn main() {

    let args = App::new("GrowBloom's Temperature").version("0.1.0")
                    .author("Anthony B.")
                    .about("Reads the temperature from a sensor and exports it.")
                    .arg(Arg::with_name("sensor")
                        .short("s")
                        .long("sensor")
                        .value_name("SENSOR")
                        .help("The sensor you want to use.")
                        .takes_value(true))
                    .arg(Arg::with_name("sensor-path")
                        .short("p")
                        .long("sensor-path")
                        .value_name("SENSOR PATH")
                        .help("The path where to read the sensor's temperature.")
                        .takes_value(true))
                    .get_matches();

    let sensor_name = args.value_of("sensor").expect("The sensor is missing.");
    let sensor_path = args.value_of("sensor-path").expect("The sensor path is missing.");

    if sensor_name == sensor::Ds18b20::NAME {
        let mut sensor = temperature::sensor::Ds18b20::new();
        sensor.path = sensor_path;
        let temperature = sensor.read();
        println!("{:?}", temperature);
    }
    

}

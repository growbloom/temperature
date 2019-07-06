# Sensors documentation

This documentation does not aim to explain much about the sensors, it supposes
you know how to assemble them on your Raspberry Pi, what temperature they can
read etc.

Instead this documentation should explain how to use a given sensor with
GrowBloom's Temperature.

## Generic rule

A sensor has two _attributes_ in GrowBloom's Temperature, two informations used
to read the datas from it and interpret them.

1. A name. Every sensor has a name, that should be given when running the
program.
2. A path. On Linux everything is a file, the sensor too.

## Sensors

- [DS18B20](./DS18B20.markdown)

## Note

If you want me to implement a given sensor, feel free to open an issue on Github.
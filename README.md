# Rust D-Bus address parsing

This project provides a Rust library for handling D-Bus addresses following the
**[D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html#addresses)**.

# Miscellaneous and caveats on D-Bus addresses

* Assumes values are UTF-8 encoded: this should be clarified in the spec
  otherwise, fail to read them or use a lossy representation for display.

* Assumes that empty `key=val` is accepted, so `transport:,,guid=...` is valid.

* Allows key only, so `transport:foo,bar` is ok.

* Accept unknown keys and transports.

# Acknowledgments

* This project is originated from the [zbus](https://github.com/dbus2/zbus) project.

* Special thanks to all the contributors who have been involved in this project.

# License

[MIT license](LICENSE-MIT)

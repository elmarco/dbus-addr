# Rust D-Bus address parsing

[![](https://docs.rs/dbus-addr/badge.svg)](https://docs.rs/dbus-addr/) [![](https://img.shields.io/crates/v/dbus-addr)](https://crates.io/crates/dbus-addr)

This project provides a Rust library for handling D-Bus addresses following the
**[D-Bus specification](https://dbus.freedesktop.org/doc/dbus-specification.html#addresses)**.

Server addresses consist of a transport name followed by a colon, and then an optional,
comma-separated list of keys and values in the form key=value.

```rust,no_run
use dbus_addr::DBusAddr;

let addr: DBusAddr = "unix:path=/tmp/dbus.sock".try_into().unwrap();
```

# Miscellaneous and caveats on D-Bus addresses

* Assumes values are UTF-8 encoded.

* Accept duplicated keys, the last one wins.

* Assumes that empty `key=val` is accepted, so `transport:,,guid=...` is valid.

* Allows key only, so `transport:foo,bar` is ok.

* Accept unknown keys and transports.

# Acknowledgments

* This project is originated from the [zbus](https://github.com/dbus2/zbus) project.

* Special thanks to all the contributors who have been involved in this project.

# License

[MIT license](LICENSE-MIT)

#![doc = include_str!("../README.md")]
use std::{env, fmt};

pub mod transport;

mod address;
pub use address::{DBusAddr, ToDBusAddrs};

mod owned_address;
pub use owned_address::{OwnedDBusAddr, ToOwnedDBusAddrs};

mod address_list;
pub use address_list::{DBusAddrList, DBusAddrListIter, OwnedDBusAddrListIter};

mod percent;
pub use percent::*;

mod guid;
pub use guid::Guid;

#[cfg(test)]
mod tests;

/// Error returned when an address is invalid.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    MissingTransport,
    Encoding(String),
    DuplicateKey(String),
    MissingKey(String),
    MissingValue(String),
    InvalidValue(String),
    UnknownTcpFamily(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingTransport => write!(f, "Missing transport in address"),
            Error::Encoding(e) => write!(f, "Encoding error: {e}"),
            Error::DuplicateKey(e) => write!(f, "Duplicate key: `{e}`"),
            Error::MissingKey(e) => write!(f, "Missing key: `{e}`"),
            Error::MissingValue(e) => write!(f, "Missing value for key: `{e}`"),
            Error::InvalidValue(e) => write!(f, "Invalid value for key: `{e}`"),
            Error::UnknownTcpFamily(e) => write!(f, "Unknown TCP address family: `{e}`"),
            Error::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Get the address for session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// $XDG_RUNTIME_DIR/bus
pub fn session() -> Result<DBusAddrList<'static>> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => DBusAddrList::try_from(val),
        _ => {
            #[cfg(windows)]
            {
                DBusAddrList::try_from("autolaunch:scope=*user;autolaunch:")
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            {
                #[link(name = "c")]
                extern "C" {
                    fn geteuid() -> u32;
                }

                let runtime_dir = env::var("XDG_RUNTIME_DIR")
                    .unwrap_or_else(|_| format!("/run/user/{}", unsafe { geteuid() }));
                let path = format!("unix:path={runtime_dir}/bus");

                DBusAddrList::try_from(path)
            }

            #[cfg(target_os = "macos")]
            {
                DBusAddrList::try_from("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET")
            }
        }
    }
}

/// Get the address for system bus respecting the DBUS_SYSTEM_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /var/run/dbus/system_bus_socket
pub fn system() -> Result<DBusAddrList<'static>> {
    match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
        Ok(val) => DBusAddrList::try_from(val),
        _ => {
            #[cfg(all(unix, not(target_os = "macos")))]
            return DBusAddrList::try_from("unix:path=/var/run/dbus/system_bus_socket");

            #[cfg(windows)]
            return DBusAddrList::try_from("autolaunch:");

            #[cfg(target_os = "macos")]
            return DBusAddrList::try_from("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
        }
    }
}

struct KeyValIter<'a> {
    data: &'a str,
    next_index: usize,
}

impl<'a> KeyValIter<'a> {
    fn new(data: &'a str) -> Self {
        KeyValIter {
            data,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for KeyValIter<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.data.len() {
            return None;
        }

        let mut pair = &self.data[self.next_index..];
        if let Some(end) = pair.find(',') {
            pair = &pair[..end];
            self.next_index += end + 1;
        } else {
            self.next_index = self.data.len();
        }
        let mut split = pair.split('=');
        // SAFETY: first split always returns something
        let key = split.next().unwrap();

        Some((key, split.next()))
    }
}

// A structure for formatting key-value pairs.
//
// This struct allows for the dynamic collection and formatting of key-value pairs,
// where keys implement `fmt::Display` and values implement `Encodable`.
pub(crate) struct KeyValFmt<'a> {
    fields: Vec<(Box<dyn fmt::Display + 'a>, Box<dyn Encodable + 'a>)>,
}

impl<'a> KeyValFmt<'a> {
    fn new() -> Self {
        Self { fields: vec![] }
    }

    pub(crate) fn add<K, V>(mut self, key: K, val: Option<V>) -> Self
    where
        K: fmt::Display + 'a,
        V: Encodable + 'a,
    {
        if let Some(val) = val {
            self.fields.push((Box::new(key), Box::new(val)));
        }

        self
    }
}

impl fmt::Display for KeyValFmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (k, v) in self.fields.iter() {
            if !first {
                write!(f, ",")?;
            }
            write!(f, "{k}=")?;
            v.encode(f)?;
            first = false;
        }

        Ok(())
    }
}

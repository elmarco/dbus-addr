use std::borrow::Cow;

use super::{percent::decode_percents_str, DBusAddr, Error, KeyValFmt, Result, TransportImpl};

/// `launchd:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-launchd>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Launchd<'a> {
    env: Cow<'a, str>,
}

impl<'a> Launchd<'a> {
    /// Environment variable.
    ///
    /// Environment variable used to get the path of the unix domain socket for the launchd created
    /// dbus-daemon.
    pub fn env(&self) -> &str {
        self.env.as_ref()
    }

    /// Convert into owned version, with 'static lifetime.
    pub fn into_owned(self) -> Launchd<'static> {
        Launchd {
            env: self.env.into_owned().into(),
        }
    }
}

impl<'a> TransportImpl<'a> for Launchd<'a> {
    fn for_address(s: &'a DBusAddr<'a>) -> Result<Self> {
        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("env", Some(v)) => {
                    return Ok(Launchd {
                        env: decode_percents_str(v)?,
                    });
                }
                _ => continue,
            }
        }

        Err(Error::MissingKey("env".into()))
    }

    fn fmt_key_val<'s: 'b, 'b>(&'s self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("env", Some(self.env()))
    }
}

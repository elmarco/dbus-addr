use std::borrow::Cow;

use super::{
    decode_percents, decode_percents_str, transport, transport::TransportImpl, Error, Guid,
    KeyValIter, Result,
};

/// A parsed bus address.
///
/// The fields of this structure are references to the source. Using an [`crate::OwnedDBusAddr`] may
/// be more convenient or if your context requires 'static lifetime.
///
/// Example:
/// ```
/// use dbus_addr::DBusAddr;
///
/// let _: DBusAddr = "unix:path=/tmp/dbus.sock".try_into().unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DBusAddr<'a> {
    pub(super) addr: Cow<'a, str>,
}

impl<'a> DBusAddr<'a> {
    /// The connection GUID if any.
    pub fn guid(&self) -> Result<Option<Guid>> {
        if let Some(guid) = self.get_string("guid") {
            Ok(Some(guid?.as_ref().try_into()?))
        } else {
            Ok(None)
        }
    }

    /// Transport connection details
    pub fn transport(&self) -> Result<transport::Transport<'_>> {
        transport::Transport::for_address(self)
    }

    /// This address as a string slice.
    pub fn as_str(&self) -> &str {
        match &self.addr {
            Cow::Borrowed(a) => a,
            Cow::Owned(a) => a.as_str(),
        }
    }

    pub(super) fn key_val_iter(&'a self) -> KeyValIter<'a> {
        let mut split = self.addr.splitn(2, ':');
        // skip transport:..
        split.next();
        let kv = split.next().unwrap_or("");
        KeyValIter::new(kv)
    }

    fn new<A: Into<Cow<'a, str>>>(addr: A) -> Result<Self> {
        let addr = addr.into();
        let addr = Self { addr };

        addr.validate()?;

        Ok(addr)
    }

    fn validate(&self) -> Result<()> {
        self.transport()?;
        for (k, v) in self.key_val_iter() {
            match (k, v) {
                ("guid", Some(v)) => {
                    Guid::try_from(decode_percents_str(v)?.as_ref())?;
                }
                (_, Some(v)) => {
                    decode_percents(v)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    // the last key=val wins
    fn get_string(&'a self, key: &str) -> Option<Result<Cow<'a, str>>> {
        let mut val = None;
        for (k, v) in self.key_val_iter() {
            if key == k {
                val = v;
            }
        }
        val.map(decode_percents_str)
    }
}

impl<'a> TryFrom<String> for DBusAddr<'a> {
    type Error = Error;

    fn try_from(addr: String) -> Result<Self> {
        Self::new(addr)
    }
}

impl<'a> TryFrom<&'a str> for DBusAddr<'a> {
    type Error = Error;

    fn try_from(addr: &'a str) -> Result<Self> {
        Self::new(addr)
    }
}

/// A trait for objects which can be converted or resolved to one or more [`DBusAddr`] values.
pub trait ToDBusAddrs<'a> {
    type Iter: Iterator<Item = Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter;
}

impl<'a> ToDBusAddrs<'a> for DBusAddr<'a> {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    /// Get an iterator over the D-Bus addresses.
    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(Ok(self.clone()))
    }
}

impl<'a> ToDBusAddrs<'a> for str {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(self.try_into())
    }
}

impl<'a> ToDBusAddrs<'a> for String {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(self.as_str().try_into())
    }
}

impl<'a> ToDBusAddrs<'a> for Vec<Result<DBusAddr<'_>>> {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Result<DBusAddr<'a>>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter {
        self.iter().cloned()
    }
}

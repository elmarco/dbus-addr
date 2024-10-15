use std::fmt;

use super::{transport, transport::TransportImpl, DBusAddr, Error, Guid, KeyValFmt, Result};

/// An owned bus address.
///
/// Example:
/// ```
/// use dbus_addr::OwnedDBusAddr;
///
/// let _: OwnedDBusAddr = "unix:path=/tmp/dbus.sock".try_into().unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnedDBusAddr {
    transport: transport::Transport<'static>,
    guid: Option<Guid>,
}

impl OwnedDBusAddr {
    /// The connection GUID if any.
    pub fn guid(&self) -> Option<&Guid> {
        self.guid.as_ref()
    }

    /// Transport connection details
    pub fn transport(&self) -> &transport::Transport<'static> {
        &self.transport
    }

    fn new(addr: &str) -> Result<Self> {
        let addr = DBusAddr { addr: addr.into() };
        let transport = addr.transport()?.into_owned();
        let guid = addr.guid()?;
        Ok(Self { transport, guid })
    }
}

impl fmt::Display for OwnedDBusAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kv = KeyValFmt::new().add("guid", self.guid.as_ref());
        let t = &self.transport;
        let kv = t.fmt_key_val(kv);
        write!(f, "{t}:{kv}")?;
        Ok(())
    }
}

impl TryFrom<&str> for OwnedDBusAddr {
    type Error = Error;

    fn try_from(addr: &str) -> Result<Self> {
        Self::new(addr)
    }
}

impl TryFrom<String> for OwnedDBusAddr {
    type Error = Error;

    fn try_from(addr: String) -> Result<Self> {
        Self::new(&addr)
    }
}

/// A trait for objects which can be converted or resolved to one or more [`OwnedDBusAddr`] values.
pub trait ToOwnedDBusAddrs<'a> {
    type Iter: Iterator<Item = Result<OwnedDBusAddr>>;

    /// Get an iterator over the D-Bus addresses.
    fn to_owned_dbus_addrs(&'a self) -> Self::Iter;
}

impl<'a> ToOwnedDBusAddrs<'a> for str {
    type Iter = std::iter::Once<Result<OwnedDBusAddr>>;

    fn to_owned_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(self.try_into())
    }
}

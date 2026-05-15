use core::{borrow::Borrow, hash::Hash, ops::Deref, str::FromStr};
use serde::{Deserialize, Serialize};
pub use smol_str::SmolStr as Str;
use std::{fmt, string::String};

/// Asset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Asset {
    inner: Str,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Parse asset error.
#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ParseAssetError {
    /// Contains the asset delimiter.
    #[error("contains `-`")]
    ContainsSep,
    /// Empty str.
    #[error("empty str cannot be asset")]
    Empty,
    /// Contains non-ascii characters.
    #[error("contains non-ascii characters")]
    NonAscii,
}

impl<'a> TryFrom<&'a str> for Asset {
    type Error = ParseAssetError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseAssetError::Empty)
        } else if !value.is_ascii() {
            Err(ParseAssetError::NonAscii)
        } else {
            Ok(Self {
                inner: Str::new(value.to_ascii_uppercase()),
            })
        }
    }
}

impl TryFrom<Str> for Asset {
    type Error = ParseAssetError;

    fn try_from(value: Str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl FromStr for Asset {
    type Err = ParseAssetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Deref for Asset {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Asset {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

impl Borrow<str> for Asset {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for &Asset {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Asset {
    /// The delimiter of assets in the spot format.
    pub const SEP: char = '-';
    /// Usdt.
    pub const USDT: Self = Self::new_inline("USDT");
    /// Usdc.
    pub const USDC: Self = Self::new_inline("USDC");
    /// Usd.
    pub const USD: Self = Self::new_inline("USD");
    /// Btc.
    pub const BTC: Self = Self::new_inline("BTC");
    /// Eth.
    pub const ETH: Self = Self::new_inline("ETH");

    /// Create a new [`Asset`] from an "inline" str.
    /// # Panic
    /// Panics if s.len() > 22.
    /// # Warning
    /// Must make sure the asset format is valid.
    const fn new_inline(s: &str) -> Self {
        Self { inner: Str::new_inline(s) }
    }

    /// Usdt.
    pub fn usdt() -> Self {
        Self::USDT
    }

    /// Usdc.
    pub fn usdc() -> Self {
        Self::USDC
    }

    /// Usd.
    pub fn usd() -> Self {
        Self::USD
    }

    /// Btc.
    pub fn btc() -> Self {
        Self::BTC
    }

    /// Eth.
    pub fn eth() -> Self {
        Self::ETH
    }

    /// Convert to [`&str`]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl PartialEq<str> for Asset {
    fn eq(&self, other: &str) -> bool {
        self.inner.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialEq<&'a str> for Asset {
    fn eq(&self, other: &&'a str) -> bool {
        self.inner.eq_ignore_ascii_case(other)
    }
}

impl PartialEq<String> for Asset {
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

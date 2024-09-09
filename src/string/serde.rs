use core::marker::PhantomData;

use serde::{de, Deserialize, Serialize};

use super::HipStr;
use crate::alloc::fmt;
use crate::alloc::string::String;
use crate::Backend;

impl<'borrow, B> Serialize for HipStr<'borrow, B>
where
    B: Backend,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de: 'borrow, 'borrow, B> Deserialize<'de> for HipStr<'borrow, B>
where
    B: Backend,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(HipStrVisitor::default())
    }
}

/// Minimal string cow visitor
struct HipStrVisitor<B: Backend> {
    phantom: PhantomData<B>,
}

impl<B: Backend> Default for HipStrVisitor<B> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<'de, B: Backend> de::Visitor<'de> for HipStrVisitor<B> {
    type Value = HipStr<'de, B>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(HipStr::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(HipStr::from(v).into_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(HipStr::from(v))
    }
}

#[cfg(test)]
mod tests;

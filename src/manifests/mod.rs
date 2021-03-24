use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{self, Formatter},
    marker::PhantomData,
};

pub mod v0_2;
pub mod v0_3;

pub struct Field<V>(String, V);

pub struct FieldMap<V>(Vec<Field<V>>);

impl<V> Default for FieldMap<V> {
    fn default() -> Self {
        FieldMap(Default::default())
    }
}

impl<V: Serialize> Serialize for FieldMap<V> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.0.iter().map(|Field(k, v)| (k, v)))
    }
}

struct FieldVisitor<V> {
    marker: PhantomData<V>,
}

impl<'de, V: Deserialize<'de>> Visitor<'de> for FieldVisitor<V> {
    type Value = FieldMap<V>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a field map")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((k, v)) = access.next_entry()? {
            vec.push(Field(k, v));
        }

        Ok(FieldMap(vec))
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for FieldMap<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(FieldVisitor {
            marker: PhantomData,
        })
    }
}

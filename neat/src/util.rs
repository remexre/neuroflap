/// Serializer/deserializer functions that use `ToString`/`FromStr`.
pub mod tofromstr {
    use std::fmt::{Display, Formatter, Result as FmtResult};
    use std::marker::PhantomData;
    use std::str::FromStr;

    use serde::de::{Deserializer, Error as DeError};
    use serde::ser::Serializer;

    /// Deserializes using `FromStr`.
    pub fn deserialize<'de, D, E, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        E: Display,
        T: FromStr<Err = E>,
    {
        struct Visitor<T>(PhantomData<T>);

        impl<'de, E, T> ::serde::de::Visitor<'de> for Visitor<T>
        where
            E: Display,
            T: FromStr<Err = E>,
        {
            type Value = T;
            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                write!(formatter, "a string")
            }

            fn visit_borrowed_str<E2>(
                self,
                v: &'de str,
            ) -> Result<Self::Value, E2>
            where
                E2: DeError,
            {
                v.parse().map_err(E2::custom)
            }
        }

        deserializer.deserialize_str(Visitor(PhantomData))
    }

    /// Serializes using `ToString`.
    pub fn serialize<S, T>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        let s = t.to_string();
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_slice, to_value, to_vec, Value};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Foo {
        #[serde(with = "super::tofromstr")]
        bar: usize,
    }

    #[test]
    fn tofromstr() {
        let foo = Foo { bar: 8 };
        let value = json!{
            {
                "bar": "8"
            }
        };

        let foo_bytes = to_vec(&foo).unwrap();
        let value_bytes = to_vec(&value).unwrap();

        let foo2: Foo = from_slice(&value_bytes).unwrap();
        let value2: Value = from_slice(&foo_bytes).unwrap();

        assert_eq!(foo_bytes, value_bytes);
        assert_eq!(foo, foo2);
        assert_eq!(value, value2);

        assert_eq!(to_value(foo).unwrap(), value);
    }
}

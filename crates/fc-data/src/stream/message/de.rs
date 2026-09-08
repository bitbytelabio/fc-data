use std::fmt;

use serde::de::{self, Deserializer, Visitor};

/// Deserializes a finite `f64` from either a JSON number or a string-encoded finite number.
///
/// Rejects non-finite values (NaN, infinity) and malformed strings.
pub(crate) fn deserialize_finite_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct FiniteF64Visitor;

    impl Visitor<'_> for FiniteF64Visitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a finite number or string-encoded finite number")
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_finite() {
                Ok(value)
            } else {
                Err(E::custom("expected a finite number, found non-finite"))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            const MAX_SAFE_INT: u64 = 1_u64 << 53;
            if value.unsigned_abs() <= MAX_SAFE_INT {
                #[allow(clippy::cast_precision_loss)]
                // Safe: integer magnitude is within 2^53, exactly representable in f64.
                Ok(value as f64)
            } else {
                Err(E::custom("integer exceeds 53-bit safe precision for f64"))
            }
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            const MAX_SAFE_INT: u64 = 1_u64 << 53;
            if value <= MAX_SAFE_INT {
                #[allow(clippy::cast_precision_loss)]
                // Safe: integer magnitude is within 2^53, exactly representable in f64.
                Ok(value as f64)
            } else {
                Err(E::custom("integer exceeds 53-bit safe precision for f64"))
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let parsed = value
                .parse::<f64>()
                .map_err(|_| E::custom(format!("invalid numeric string {value:?}")))?;
            if parsed.is_finite() {
                Ok(parsed)
            } else {
                Err(E::custom(
                    "expected a finite number in string, found non-finite",
                ))
            }
        }
    }

    deserializer.deserialize_any(FiniteF64Visitor)
}

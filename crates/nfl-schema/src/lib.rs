//! NFL schema metadata and storage-compatible field definitions.
#![deny(missing_docs)]

use std::borrow::Cow;
use std::fmt;

use nfl_core::Result;

/// Supported field types for NFL schema metadata.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    Int32 = 1,
    Int64 = 2,
    Float32 = 3,
    Float64 = 4,
    Utf8 = 5,
    Binary = 6,
    Vector = 7,
    Tensor = 8,
}

impl FieldType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            1 => Ok(FieldType::Int32),
            2 => Ok(FieldType::Int64),
            3 => Ok(FieldType::Float32),
            4 => Ok(FieldType::Float64),
            5 => Ok(FieldType::Utf8),
            6 => Ok(FieldType::Binary),
            7 => Ok(FieldType::Vector),
            8 => Ok(FieldType::Tensor),
            _ => Err(nfl_core::Error::Corrupted("unknown field type")),
        }
    }
}

/// A named field in the NFL schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
}

impl Field {
    pub fn new(name: impl Into<String>, field_type: FieldType, nullable: bool) -> Self {
        Self {
            name: name.into(),
            field_type,
            nullable,
        }
    }
}

/// A schema describing named fields and their storage types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub fields: Vec<Field>,
}

impl Schema {
    /// Create a new schema from fields.
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    /// Serialize the schema into a compact binary representation.
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(1); // schema format version
        out.push(self.fields.len() as u8);

        for field in &self.fields {
            let name = field.name.as_bytes();
            out.extend_from_slice(&(name.len() as u16).to_le_bytes());
            out.push(field.field_type as u8);
            out.push(field.nullable as u8);
            out.extend_from_slice(name);
        }

        out
    }

    /// Parse a schema from bytes.
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 2 {
            return Err(nfl_core::Error::InsufficientData);
        }

        let version = bytes[0];
        if version != 1 {
            return Err(nfl_core::Error::UnsupportedVersion(version as u32));
        }

        let field_count = bytes[1] as usize;
        let mut cursor = 2;
        let mut fields = Vec::with_capacity(field_count);

        for _ in 0..field_count {
            if cursor + 4 > bytes.len() {
                return Err(nfl_core::Error::InsufficientData);
            }

            let name_len = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]) as usize;
            cursor += 2;
            let field_type = FieldType::from_u8(bytes[cursor])?;
            cursor += 1;
            let nullable = bytes[cursor] != 0;
            cursor += 1;

            if cursor + name_len > bytes.len() {
                return Err(nfl_core::Error::InsufficientData);
            }

            let name = String::from_utf8(bytes[cursor..cursor + name_len].to_vec())
                .map_err(|_| nfl_core::Error::Corrupted("invalid UTF-8 in field name"))?;
            cursor += name_len;
            fields.push(Field::new(name, field_type, nullable));
        }

        Ok(Schema { fields })
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FieldType::Int32 => "Int32",
            FieldType::Int64 => "Int64",
            FieldType::Float32 => "Float32",
            FieldType::Float64 => "Float64",
            FieldType::Utf8 => "Utf8",
            FieldType::Binary => "Binary",
            FieldType::Vector => "Vector",
            FieldType::Tensor => "Tensor",
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_serialize_parse_roundtrip() {
        let schema = Schema::new(vec![
            Field::new("id", FieldType::Int64, false),
            Field::new("name", FieldType::Utf8, true),
            Field::new("embedding", FieldType::Vector, false),
        ]);

        let frame = schema.serialize();
        let parsed = Schema::parse(&frame).expect("parse schema");

        assert_eq!(parsed, schema);
    }

    #[test]
    fn parse_invalid_version() {
        let bytes = [0u8, 0u8];
        assert!(matches!(Schema::parse(&bytes), Err(nfl_core::Error::UnsupportedVersion(_))));
    }
}

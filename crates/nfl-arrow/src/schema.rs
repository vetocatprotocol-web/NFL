//! Arrow schema interoperability for NFL schema metadata.

use arrow2::datatypes::{DataType as ArrowDataType, Field as ArrowField, Schema as ArrowSchema};
use nfl_core::Result;
use nfl_schema::{Field, FieldType, Schema};

/// Extension methods for converting NFL schema metadata into Arrow schemas.
pub trait SchemaArrowExt {
    /// Convert the NFL schema into an Arrow schema representation.
    fn to_arrow_schema(&self) -> ArrowSchema;
}

impl SchemaArrowExt for Schema {
    fn to_arrow_schema(&self) -> ArrowSchema {
        let fields = self
            .fields
            .iter()
            .map(|field| {
                let data_type = match field.field_type {
                    FieldType::Int32 => ArrowDataType::Int32,
                    FieldType::Int64 => ArrowDataType::Int64,
                    FieldType::Float32 => ArrowDataType::Float32,
                    FieldType::Float64 => ArrowDataType::Float64,
                    FieldType::Utf8 => ArrowDataType::Utf8,
                    FieldType::Binary => ArrowDataType::LargeBinary,
                    FieldType::Vector => ArrowDataType::LargeBinary,
                    FieldType::Tensor => ArrowDataType::LargeBinary,
                };
                ArrowField::new(&field.name, data_type, field.nullable)
            })
            .collect();

        ArrowSchema::new(fields)
    }
}

/// Extension methods for converting Arrow schemas to NFL schema metadata.
pub trait ArrowSchemaArrowExt {
    /// Convert an Arrow schema into an NFL schema.
    fn to_nfl_schema(&self) -> Result<Schema>;
}

impl ArrowSchemaArrowExt for ArrowSchema {
    fn to_nfl_schema(&self) -> Result<Schema> {
        let mut fields = Vec::with_capacity(self.fields.len());

        for arrow_field in &self.fields {
            let field_type = match &arrow_field.data_type {
                ArrowDataType::Int32 => FieldType::Int32,
                ArrowDataType::Int64 => FieldType::Int64,
                ArrowDataType::Float32 => FieldType::Float32,
                ArrowDataType::Float64 => FieldType::Float64,
                ArrowDataType::Utf8 => FieldType::Utf8,
                ArrowDataType::LargeBinary => FieldType::Binary,
                ArrowDataType::Binary => FieldType::Binary,
                _ => {
                    return Err(nfl_core::Error::Corrupted("unsupported Arrow data type"));
                }
            };
            fields.push(Field::new(
                arrow_field.name.clone(),
                field_type,
                arrow_field.nullable,
            ));
        }

        Ok(Schema::new(fields))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow2::datatypes::{DataType as ArrowDataType, Field as ArrowField, Schema as ArrowSchema};
    use nfl_schema::{Field, FieldType, Schema};

    #[test]
    fn schema_to_arrow_roundtrip() {
        let schema = Schema::new(vec![
            Field::new("id", FieldType::Int64, false),
            Field::new("name", FieldType::Utf8, true),
            Field::new("blob", FieldType::Binary, false),
        ]);

        let arrow_schema = schema.to_arrow_schema();
        assert_eq!(arrow_schema.fields.len(), 3);
        assert_eq!(arrow_schema.fields[0].data_type, ArrowDataType::Int64);
        assert_eq!(arrow_schema.fields[1].data_type, ArrowDataType::Utf8);
        assert_eq!(arrow_schema.fields[2].data_type, ArrowDataType::LargeBinary);

        let parsed = arrow_schema.to_nfl_schema().expect("convert back");
        assert_eq!(parsed, schema);
    }

    #[test]
    fn unsupported_arrow_type_rejected() {
        let arrow_schema = ArrowSchema::new(vec![ArrowField::new("flag", ArrowDataType::Boolean, false)]);
        assert!(arrow_schema.to_nfl_schema().is_err());
    }
}

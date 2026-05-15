use util_validation::{ValidateFrom, Validated, ValidationError};
use uuid::Uuid;
use yooso_core::error::Result;

/// Corresponds to a field of a component in the application.
#[collection(db = crate::MetaDB, table = "fields")]
#[derive(Default)]
#[unique(component_id, field_name)]
#[unique(component_id, position)]
pub struct ComponentFieldTable {
    /// Snowflake value. This is the unique identifier of the field.
    #[primary]
    pub id: Uuid,

    /// The ID of the component that this field belongs to.
    pub component_id: Uuid,

    /// The name of the field.
    pub field_name: String,

    /// The type of the field, represented as a string.
    pub field_type: String,

    /// Whether the field is system (true) or user-defined (false).
    pub is_system: bool,

    /// The order index of the field. This is used in the admin panel to
    /// preserve the field order and has no functional significance in the
    /// application logic.
    pub position: i32,

    /// The timestamp of when the component was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}

impl ComponentFieldTable {
    /// Lists all components for a given component ID.
    pub async fn list_by_component_id(
        db: &crate::MetaDBState,
        component_id: &Uuid,
    ) -> Result<Vec<Self>, ::yooso_core::Error> {
        let conn = db.0.lock().map_err(|e| ::yooso_core::Error::from(e))?;

        let mut stmt = conn
            .prepare("SELECT * FROM fields WHERE component_id = ?")
            .map_err(|e| ::yooso_core::Error::from(e))?;

        stmt.query_map(::rusqlite::params![component_id.to_string()], |row| {
            Ok(Self {
                id: Uuid::parse_str(&row.get::<_, String>(0)?)
                    .map_err(|_| ::rusqlite::Error::InvalidQuery)?,
                component_id: Uuid::parse_str(&row.get::<_, String>(1)?)
                    .map_err(|_| ::rusqlite::Error::InvalidQuery)?,
                field_name: row.get(2)?,
                field_type: row.get(3)?,
                is_system: row.get(4)?,
                position: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| ::yooso_core::Error::from(e))?
        .collect::<Result<Vec<_>, ::rusqlite::Error>>()
        .map_err(|e| ::yooso_core::Error::from(e))
    }
}

impl ValidateFrom<Self> for ComponentFieldTable {
    fn validate(input: Self) -> Result<Validated<Self>, ValidationError> {
        let field_name = &input.field_name;
        let field_type = &input.field_type;

        util_validation::not_empty(field_name)
            .map_err(|e| e.prepend("Field name"))?;
        util_validation::valid_sql_ident(field_name)
            .map_err(|e| e.prepend(format!("Field `{field_name}`")))?;
        util_validation::not_sql_keyword(field_name)
            .map_err(|e| e.prepend(format!("Field name")))?;

        // TODO rewrite to proper type checking (when we have proper types)
        util_validation::not_empty(field_type)
            .map_err(|e| e.prepend(format!("Field `{field_name}` type")))?;
        util_validation::valid_sql_ident(field_type)
            .map_err(|e| e.prepend(format!("Field `{field_name}` type")))?;
        util_validation::not_sql_keyword(field_type)
            .map_err(|e| e.prepend(format!("Field `{field_name}` type")))?;

        Ok(Validated(input))
    }
}
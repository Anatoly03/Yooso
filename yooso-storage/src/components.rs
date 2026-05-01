use crate::ComponentFieldTable;
use rusqlite::types::ValueRef;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;
use yooso_core::Error::ValidationError;
use yooso_core::error::Result;

/// Corresponds to a component in the application.
#[collection(db = crate::MetaDB, table = "components")]
#[unique(component_name)]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ComponentTable {
    /// Snowflake value. This is the unique identifier of the component.
    #[primary]
    pub id: Uuid,

    /// The name of the component.
    pub component_name: String,

    /// The color of the component, represented in RGB0 integer format. This
    /// is used in the admin panel to visually distinguish components and has
    /// no functional significance in the application logic.
    pub color: u32,

    /// Whether the component is a system component (true) or a user-defined
    /// component (false).
    pub is_system: bool,

    /// The timestamp of when the component was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}

impl ComponentTable {
    /// If an entity with the given component ID exists in the database,
    /// returns a JSON object containing the component's data and its fields,
    /// or an error if the component or entity is not found.
    pub async fn for_entity(
        &self,
        state: &crate::MetaDBState,
        general_state: &crate::GeneralDBState,
        id: &Uuid,
    ) -> Result<Value> {
        // First, we need to retrieve the component's fields from
        // the meta database. The fields are also the keys of the
        // JSON object that we return.
        let fields = self.schema(state).await?;

        // Retrieves the row from the general database that corresponds
        // to this component and the given entity ID.
        let conn = general_state
            .0
            .lock()
            .map_err(|e| ::yooso_core::Error::from(e))?;

        conn.query_row(
            &format!("SELECT * FROM {} WHERE entity_id = ?", self.component_name),
            rusqlite::params![id.to_string()],
            |row| {
                let mut obj = json!({});

                for (i, field) in fields.iter().enumerate() {
                    let value = row
                        .get_ref(i + 1)
                        .map(sql_value_to_json)
                        .unwrap_or(Value::Null);
                    obj[field.field_name.clone()] = value;
                }

                Ok(obj)
            },
        )
        .map_err(|e| ::yooso_core::Error::from(e))
    }

    /// Wether an entity implements the current component.
    pub async fn defined_for(
        &self,
        general_state: &crate::GeneralDBState,
        entity_id: &Uuid,
    ) -> Result<bool> {
        let query = format!(
            "SELECT EXISTS(SELECT 1 FROM \"{}\" WHERE entity_id = '{}' LIMIT 1)",
            self.component_name, entity_id
        );

        let conn = general_state
            .0
            .lock()
            .map_err(|e| yooso_core::Error::from(e))?;

        if cfg!(debug_assertions) {
            eprintln!("\x1b[90m{query}\x1b[0m");
        }

        let exists = conn
            .query_row(&query, [], |row| row.get::<_, i64>(0))
            .map_err(|e| yooso_core::Error::from(e))?;

        Ok(exists == 1)
    }

    /// Validates the component metadata.
    pub fn validate(&self) -> Result<()> {
        crate::validate::not_empty(&self.component_name, "component name")?;
        crate::validate::valid_sql_ident(&self.component_name, "component name")?;
        crate::validate::not_sql_keyword(&self.component_name, "component name")?;

        Ok(())
    }

    /// Retrieves the component schema. Invokes the [ComponentFieldTable::list_by_component_id]
    /// function to get the fields of this component.
    pub async fn schema(&self, state: &crate::MetaDBState) -> Result<Vec<ComponentFieldTable>> {
        ComponentFieldTable::list_by_component_id(state, &self.id).await
    }
}

/// Converts a SQLite value to a JSON value.
fn sql_value_to_json(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(v) => Value::from(v),
        ValueRef::Real(v) => Value::from(v),
        ValueRef::Text(v) => Value::from(String::from_utf8_lossy(v).to_string()),
        ValueRef::Blob(v) => Value::from(v.to_vec()),
    }
}

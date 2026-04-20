use crate::ComponentFieldTable;
use rusqlite::types::ValueRef;
use serde_json::{Value, json};
use uuid::Uuid;

/// Represents a table in the database that corresponds to a component in the application.
#[collection(db = crate::MetaDB, table = "components")]
#[unique(component_name)]
#[derive(Default)]
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
    /// or [None] if no such component exists.
    pub async fn for_entity(
        &self,
        state: &crate::MetaDBState,
        general_state: &crate::GeneralDBState,
        id: &Uuid,
    ) -> Option<Value> {
        // First, we need to retrieve the component's fields from
        // the meta database. The fields are also the keys of the
        // JSON object that we return.
        let fields = self.schema(state).await;

        // Retrieves the row from the general database that corresponds
        // to this component and the given entity ID.
        general_state
            .0
            .lock()
            .unwrap()
            .query_row(
                &format!("SELECT * FROM {} WHERE id = ?", self.component_name),
                rusqlite::params![id.to_string()],
                |row| {
                    let mut obj = json!({});

                    for (i, field) in fields.iter().enumerate() {
                        let value = row.get_ref(i + 1).map(sql_value_to_json).unwrap_or(Value::Null);
                        obj[field.field_name.clone()] = value;
                    }

                    Ok(obj)
                },
            )
            .ok()
    }

    /// Retrieves the component schema. Invokes the [ComponentFieldTable::list_by_component_id]
    /// function to get the fields of this component.
    pub async fn schema(&self, state: &crate::MetaDBState) -> Vec<ComponentFieldTable> {
        ComponentFieldTable::list_by_component_id(state, &self.id)
            .await
            .expect("failed to view component fields")
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

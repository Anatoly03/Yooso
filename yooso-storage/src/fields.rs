use uuid::Uuid;
use yooso_macro::collection;

/// Represents a table in the database that corresponds to a component in the application.
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
    ) -> Result<Vec<Self>, ::rusqlite::Error> {
        let conn = db.0.lock().expect("lock db mutex");

        let mut stmt = conn.prepare("SELECT * FROM fields WHERE component_id = ?")?;

        stmt.query_map(::rusqlite::params![component_id.to_string()], |row| {
            Ok(Self {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).expect("failed to parse uuid"),
                component_id: Uuid::parse_str(&row.get::<_, String>(1)?)
                    .expect("failed to parse component uuid"),
                field_name: row.get(2)?,
                field_type: row.get(3)?,
                is_system: row.get(4)?,
                position: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
    }
}

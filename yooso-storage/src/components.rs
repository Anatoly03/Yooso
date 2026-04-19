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

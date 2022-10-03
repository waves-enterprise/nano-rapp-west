/// Structure that sets key and value of account data storage entry.
pub enum DataEntry {
    Integer(u64),
    Boolean(bool),
    Binary,
    String,
    Undefined,
}

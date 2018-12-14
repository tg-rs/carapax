/// Contains information about why a request was unsuccessful.
#[derive(Debug)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier.
    migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control,
    /// the number of seconds left to wait
    /// before the request can be repeated
    retry_after: Option<i64>,
}

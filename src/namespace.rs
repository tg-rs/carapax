use carapax::core::types::Update;

/// A session namespace
#[derive(Clone)]
pub struct SessionNamespace {
    namespace: String,
}

impl SessionNamespace {
    /// Creates a new namespace with given prefix
    pub fn new<S: Into<String>>(namespace: S) -> Self {
        Self {
            namespace: namespace.into(),
        }
    }

    pub(crate) fn from_update(update: &Update) -> Self {
        let namespace = match (update.get_chat_id(), update.get_user().map(|x| x.id)) {
            (Some(chat_id), Some(user_id)) => format!("{}-{}", chat_id, user_id),
            (Some(chat_id), None) => format!("{}-{}", chat_id, chat_id),
            (None, Some(user_id)) => format!("{}-{}", user_id, user_id),
            (None, None) => unreachable!(), // there is always either user_id or chat_id
        };
        Self { namespace }
    }

    /// Returns a new key with current namespace and `-` as delimiter
    pub fn format_key(&self, key: &str) -> String {
        format!("{}-{}", self.namespace, key)
    }
}

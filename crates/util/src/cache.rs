use std::sync::OnceLock;

static INSTANCE: OnceLock<redis::Client> = OnceLock::new();

use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

pub static AUTH_COOKIE_NAME: &str = "one-plan-id";

/// Create a new v4 UUID string
pub fn new_id() -> String {
    String::from(
        Uuid::new_v4()
            .as_simple()
            .encode_lower(&mut Uuid::encode_buffer()),
    )
}

pub fn create_password_hash(password: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash::<&str>(password, bcrypt::DEFAULT_COST)?)
}

/// Get current UTC time without Timezone info
pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_id() {
        assert_ne!(super::new_id(), super::new_id())
    }

    #[test]
    fn now() {
        let t1 = super::now();
        let t2 = super::now();
        assert_eq!(t1.date(), t2.date());
        assert_eq!(t1.timestamp_millis(), t2.timestamp_millis());
        // time should change between every call
        assert_ne!(t1, t2);
    }
}

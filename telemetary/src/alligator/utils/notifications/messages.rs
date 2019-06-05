pub(crate) const NOTIFICATION_ROUTE: &str = "/notify";
use crate::alligator::utils::notifications::NotificationTypes;
use serde_json::{json, Value};

pub(crate) fn notify_message(content: Value, notification_type: NotificationTypes) -> String {
    json!({
        "route": NOTIFICATION_ROUTE,
        "notification":{
            "type": notification_type,
            "content":content.to_string()
        }
    })
    .to_string()
}

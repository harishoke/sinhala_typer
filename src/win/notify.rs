use notify_rust::Notification;

/// Show a desktop toast. Best-effort: failures are ignored so a missing
/// notification service never disturbs typing.
pub fn show(title: &str, body: &str) {
    let _ = Notification::new().summary(title).body(body).show();
}

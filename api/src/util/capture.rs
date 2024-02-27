/// Macro to capture and log an error message.
///
/// This macro captures the given error message using Sentry for error monitoring purposes.
/// It also logs the error using the `logw` crate to output the error message to the log.
///
/// # Note
/// I'm not sure if I like this approach, but it's good enough for this small project.
#[macro_export]
macro_rules! capture_error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);

        sentry::capture_message(&message, sentry::Level::Error);
        logw::tracing::error!("{}", &message)
    }};
}
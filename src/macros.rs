#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use std::io::Write;
        if let Ok(mut writer) = crate::statics::LOG_FILE.lock() {
            let _ = writeln!(writer, $($arg)*);
            let _ = writer.flush();
        }
    }};
}
#[macro_export]
macro_rules! try_or_return {
    // Case with optional side-effect block before return
    ($expr:expr, $on_err:expr) => {
        match $expr {
            Ok(val) => val,
            Err(resp) => {
                $on_err;
                return resp;
            }
        }
    };
    // Fallback case without side-effect
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(resp) => return resp,
        }
    };
}

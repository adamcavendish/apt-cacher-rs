#[macro_export]
macro_rules! warn_once {
    ($($t:tt)*) => {{
        static FIRED: std::sync::OnceLock<std::marker::PhantomData<bool>> =
            std::sync::OnceLock::new();
        FIRED.get_or_init(|| {
            log::warn!($($t)*);
            std::marker::PhantomData
        });
    }};
}

#[macro_export]
macro_rules! warn_once_or_info {
    ($($t:tt)*) => {{
        static FIRED: std::sync::OnceLock<std::marker::PhantomData<bool>> =
            std::sync::OnceLock::new();
        match FIRED.set(std::marker::PhantomData) {
            Ok(()) => log::warn!($($t)*),
            Err(_) => log::info!($($t)*),
        }

    }};
}

#[macro_export]
macro_rules! info_once {
    ($($t:tt)*) => {{
        static FIRED: std::sync::OnceLock<std::marker::PhantomData<bool>> =
            std::sync::OnceLock::new();
        FIRED.get_or_init(|| {
            log::info!($($t)*);
            std::marker::PhantomData
        });
    }};
}

pub use encrypt_macro;

#[macro_export]
macro_rules! encrypt {
    ($str:literal) => {
        $crate::encrypt_macro::encrypt!($str);
    };
}

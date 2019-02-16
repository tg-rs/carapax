macro_rules! impl_enum_from {
    (
        $target:ident { $($to:ident($from:ty)),* }
    ) => {
        $(
            impl From<$from> for $target {
                fn from(obj: $from) -> $target {
                    $target::$to(obj.into())
                }
            }
        )*
    };
}

// See https://internals.rust-lang.org/t/announcing-rust-2018-beta-release/8901/17?u=johnthagen
pub(crate) use impl_enum_from;

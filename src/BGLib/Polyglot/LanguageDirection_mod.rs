#[cfg(feature = "BGLib+Polyglot+LanguageDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LanguageDirection {
    #[default]
    LeftToRight = 0i32,
    RightToLeft = 1i32,
}
#[cfg(feature = "BGLib+Polyglot+LanguageDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LanguageDirection =>
    "BGLib.Polyglot"."LanguageDirection"
);

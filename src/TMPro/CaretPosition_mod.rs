#[cfg(feature = "TMPro+CaretPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CaretPosition {
    #[default]
    Left = 1i32,
    None = 0i32,
    Right = 2i32,
}
#[cfg(feature = "TMPro+CaretPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::CaretPosition => "TMPro"."CaretPosition"
);

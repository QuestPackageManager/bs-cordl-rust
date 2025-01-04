#[cfg(feature = "TMPro+TMP_TextElementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TMP_TextElementType {
    #[default]
    Character = 0i32,
    Sprite = 1i32,
}
#[cfg(feature = "TMPro+TMP_TextElementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_TextElementType => "TMPro"
    ."TMP_TextElementType"
);

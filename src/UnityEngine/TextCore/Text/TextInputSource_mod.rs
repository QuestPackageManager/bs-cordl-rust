#[cfg(feature = "UnityEngine+TextCore+Text+TextInputSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextInputSource {
    #[default]
    SetText = 1i32,
    SetTextArray = 2i32,
    TextInputBox = 0i32,
    TextString = 3i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextInputSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextInputSource =>
    "UnityEngine.TextCore.Text"."TextInputSource"
);

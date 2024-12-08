#[cfg(feature = "ToneMapping")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneMapping {
    Aces = 1i32,
    None = 0i32,
}
#[cfg(feature = "ToneMapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ToneMapping => ""."ToneMapping"
);

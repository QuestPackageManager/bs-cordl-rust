#[cfg(feature = "BeatSaber+Init+RuntimePlatformType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuntimePlatformType {
    #[default]
    PS4 = 0i32,
    PS5 = 1i32,
    Quest = 2i32,
    Rift = 3i32,
    Steam = 4i32,
}
#[cfg(feature = "BeatSaber+Init+RuntimePlatformType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::RuntimePlatformType =>
    "BeatSaber.Init"."RuntimePlatformType"
);

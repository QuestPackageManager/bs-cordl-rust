#[cfg(feature = "Unity+Burst+GlobalSafetyChecksSettingKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlobalSafetyChecksSettingKind {
    #[default]
    ForceOn = 2i32,
    Off = 0i32,
    On = 1i32,
}
#[cfg(feature = "Unity+Burst+GlobalSafetyChecksSettingKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::GlobalSafetyChecksSettingKind =>
    "Unity.Burst"."GlobalSafetyChecksSettingKind"
);

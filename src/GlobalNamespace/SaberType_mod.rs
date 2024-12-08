#[cfg(feature = "SaberType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaberType {
    SaberA = 0i32,
    SaberB = 1i32,
}
#[cfg(feature = "SaberType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for SaberType => ""."SaberType"
);

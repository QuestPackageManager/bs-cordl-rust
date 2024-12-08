#[cfg(feature = "Oculus+Platform+InitConfigOptions")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitConfigOptions {
    DisableP2pNetworking = 3730693852u32,
}
#[cfg(feature = "Oculus+Platform+InitConfigOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::InitConfigOptions =>
    "Oculus.Platform"."InitConfigOptions"
);

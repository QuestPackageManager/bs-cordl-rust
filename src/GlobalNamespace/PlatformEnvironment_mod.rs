#[cfg(feature = "PlatformEnvironment")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformEnvironment {
    Certification = 1u8,
    Development = 0u8,
    Production = 2u8,
}
#[cfg(feature = "PlatformEnvironment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for PlatformEnvironment => ""."PlatformEnvironment"
);

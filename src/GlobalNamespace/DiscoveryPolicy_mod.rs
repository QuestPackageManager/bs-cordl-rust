#[cfg(feature = "DiscoveryPolicy")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryPolicy {
    Hidden = 0u8,
    Public = 2u8,
    WithCode = 1u8,
}
#[cfg(feature = "DiscoveryPolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DiscoveryPolicy => ""
    ."DiscoveryPolicy"
);

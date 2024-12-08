#[cfg(feature = "Mono+Security+Interface+AlertLevel")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    Fatal = 2u8,
    Warning = 1u8,
}
#[cfg(feature = "Mono+Security+Interface+AlertLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::AlertLevel =>
    "Mono.Security.Interface"."AlertLevel"
);

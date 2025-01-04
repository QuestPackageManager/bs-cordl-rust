#[cfg(feature = "UnityEngine+Timeline+NotificationFlags")]
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationFlags {
    #[default]
    Retroactive = 2i16,
    TriggerInEditMode = 1i16,
    TriggerOnce = 4i16,
}
#[cfg(feature = "UnityEngine+Timeline+NotificationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::NotificationFlags =>
    "UnityEngine.Timeline"."NotificationFlags"
);

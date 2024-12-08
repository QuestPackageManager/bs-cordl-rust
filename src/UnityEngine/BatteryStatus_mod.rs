#[cfg(feature = "UnityEngine+BatteryStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging = 1i32,
    Discharging = 2i32,
    Full = 4i32,
    NotCharging = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+BatteryStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BatteryStatus => "UnityEngine"
    ."BatteryStatus"
);

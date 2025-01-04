#[cfg(feature = "OVR+OpenVR+EVRState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRState {
    #[default]
    NotReady = 5i32,
    Off = 0i32,
    Ready = 3i32,
    Ready_Alert = 4i32,
    Ready_Alert_Low = 7i32,
    Searching = 1i32,
    Searching_Alert = 2i32,
    Standby = 6i32,
    Undefined = -1i32,
}
#[cfg(feature = "OVR+OpenVR+EVRState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRState => "OVR.OpenVR"."EVRState"
);

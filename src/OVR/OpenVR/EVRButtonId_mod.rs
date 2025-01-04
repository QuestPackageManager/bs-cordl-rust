#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRButtonId {
    #[default]
    k_EButton_A = 7i32,
    k_EButton_ApplicationMenu = 1i32,
    k_EButton_Axis0 = 32i32,
    k_EButton_Axis1 = 33i32,
    k_EButton_Axis2 = 34i32,
    k_EButton_Axis3 = 35i32,
    k_EButton_Axis4 = 36i32,
    k_EButton_DPad_Down = 6i32,
    k_EButton_DPad_Left = 3i32,
    k_EButton_DPad_Right = 5i32,
    k_EButton_DPad_Up = 4i32,
    k_EButton_Dashboard_Back = 2i32,
    k_EButton_Max = 64i32,
    k_EButton_ProximitySensor = 31i32,
    k_EButton_System = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVRButtonId => "OVR.OpenVR"
    ."EVRButtonId"
);

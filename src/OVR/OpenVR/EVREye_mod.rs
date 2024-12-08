#[cfg(feature = "OVR+OpenVR+EVREye")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVREye {
    Eye_Left = 0i32,
    Eye_Right = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EVREye")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EVREye => "OVR.OpenVR"."EVREye"
);

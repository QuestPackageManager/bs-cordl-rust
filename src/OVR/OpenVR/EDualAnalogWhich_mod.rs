#[cfg(feature = "OVR+OpenVR+EDualAnalogWhich")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EDualAnalogWhich {
    k_EDualAnalog_Left = 0i32,
    k_EDualAnalog_Right = 1i32,
}
#[cfg(feature = "OVR+OpenVR+EDualAnalogWhich")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EDualAnalogWhich => "OVR.OpenVR"
    ."EDualAnalogWhich"
);

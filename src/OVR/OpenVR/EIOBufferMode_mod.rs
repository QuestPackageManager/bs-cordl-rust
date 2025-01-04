#[cfg(feature = "OVR+OpenVR+EIOBufferMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EIOBufferMode {
    #[default]
    Create = 512i32,
    Read = 1i32,
    Write = 2i32,
}
#[cfg(feature = "OVR+OpenVR+EIOBufferMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EIOBufferMode => "OVR.OpenVR"
    ."EIOBufferMode"
);

#[cfg(feature = "OVR+OpenVR+EVROverlayIntersectionMaskPrimitiveType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVROverlayIntersectionMaskPrimitiveType {
    #[default]
    OverlayIntersectionPrimitiveType_Circle = 1i32,
    OverlayIntersectionPrimitiveType_Rectangle = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVROverlayIntersectionMaskPrimitiveType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::OVR::OpenVR::EVROverlayIntersectionMaskPrimitiveType => "OVR.OpenVR"
    ."EVROverlayIntersectionMaskPrimitiveType"
);

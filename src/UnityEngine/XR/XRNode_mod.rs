#[cfg(feature = "UnityEngine+XR+XRNode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XRNode {
    CenterEye = 2i32,
    GameController = 6i32,
    HardwareTracker = 8i32,
    Head = 3i32,
    LeftEye = 0i32,
    LeftHand = 4i32,
    RightEye = 1i32,
    RightHand = 5i32,
    TrackingReference = 7i32,
}
#[cfg(feature = "UnityEngine+XR+XRNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRNode => "UnityEngine.XR"
    ."XRNode"
);

#[cfg(feature = "OVRGLTFInputNode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRGLTFInputNode {
    #[default]
    Button_A_X = 1i32,
    Button_B_Y = 2i32,
    Button_Oculus_Menu = 3i32,
    None = 0i32,
    ThumbStick = 6i32,
    Trigger_Front = 5i32,
    Trigger_Grip = 4i32,
}
#[cfg(feature = "OVRGLTFInputNode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGLTFInputNode => ""
    ."OVRGLTFInputNode"
);

#[cfg(feature = "NodeStatePropertyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeStatePropertyType {
    #[default]
    Acceleration = 0i32,
    AngularAcceleration = 1i32,
    AngularVelocity = 3i32,
    Orientation = 5i32,
    Position = 4i32,
    Velocity = 2i32,
}
#[cfg(feature = "NodeStatePropertyType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NodeStatePropertyType => ""
    ."NodeStatePropertyType"
);

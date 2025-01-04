#[cfg(feature = "UnityEngine+RotationOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RotationOrder {
    #[default]
    OrderXYZ = 0i32,
    OrderXZY = 1i32,
    OrderYXZ = 3i32,
    OrderYZX = 2i32,
    OrderZXY = 4i32,
    OrderZYX = 5i32,
}
#[cfg(feature = "UnityEngine+RotationOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RotationOrder => "UnityEngine"
    ."RotationOrder"
);

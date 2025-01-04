#[cfg(feature = "UnityEngine+ProBuilder+ShapeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShapeType {
    #[default]
    Arch = 10i32,
    Cone = 8i32,
    Cube = 0i32,
    CurvedStair = 2i32,
    Cylinder = 4i32,
    Door = 6i32,
    Pipe = 7i32,
    Plane = 5i32,
    Prism = 3i32,
    Sphere = 11i32,
    Sprite = 9i32,
    Stair = 1i32,
    Torus = 12i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ShapeType =>
    "UnityEngine.ProBuilder"."ShapeType"
);

#[cfg(feature = "UnityEngine+SkinQuality")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkinQuality {
    #[default]
    Auto = 0i32,
    Bone1 = 1i32,
    Bone2 = 2i32,
    Bone4 = 4i32,
}
#[cfg(feature = "UnityEngine+SkinQuality")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SkinQuality => "UnityEngine"
    ."SkinQuality"
);

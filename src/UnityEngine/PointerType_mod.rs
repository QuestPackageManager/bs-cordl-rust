#[cfg(feature = "UnityEngine+PointerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PointerType {
    #[default]
    Mouse = 0i32,
    Pen = 2i32,
    Touch = 1i32,
}
#[cfg(feature = "UnityEngine+PointerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PointerType => "UnityEngine"
    ."PointerType"
);

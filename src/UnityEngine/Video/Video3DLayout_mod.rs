#[cfg(feature = "UnityEngine+Video+Video3DLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Video3DLayout {
    #[default]
    No3D = 0i32,
    OverUnder3D = 2i32,
    SideBySide3D = 1i32,
}
#[cfg(feature = "UnityEngine+Video+Video3DLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::Video3DLayout =>
    "UnityEngine.Video"."Video3DLayout"
);

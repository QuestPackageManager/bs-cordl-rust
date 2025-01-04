#[cfg(feature = "TMPro+TMP_VertexDataUpdateFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TMP_VertexDataUpdateFlags {
    #[default]
    All = 255i32,
    Colors32 = 16i32,
    None = 0i32,
    Uv0 = 2i32,
    Uv2 = 4i32,
    Uv4 = 8i32,
    Vertices = 1i32,
}
#[cfg(feature = "TMPro+TMP_VertexDataUpdateFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_VertexDataUpdateFlags => "TMPro"
    ."TMP_VertexDataUpdateFlags"
);

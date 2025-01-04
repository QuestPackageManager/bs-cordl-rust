#[cfg(feature = "TMPro+TextRenderFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextRenderFlags {
    #[default]
    DontRender = 0i32,
    Render = 255i32,
}
#[cfg(feature = "TMPro+TextRenderFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TextRenderFlags => "TMPro"
    ."TextRenderFlags"
);

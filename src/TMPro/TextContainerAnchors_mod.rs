#[cfg(feature = "TMPro+TextContainerAnchors")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextContainerAnchors {
    Bottom = 7i32,
    BottomLeft = 6i32,
    BottomRight = 8i32,
    Custom = 9i32,
    Left = 3i32,
    Middle = 4i32,
    Right = 5i32,
    Top = 1i32,
    TopLeft = 0i32,
    TopRight = 2i32,
}
#[cfg(feature = "TMPro+TextContainerAnchors")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TextContainerAnchors => "TMPro"
    ."TextContainerAnchors"
);

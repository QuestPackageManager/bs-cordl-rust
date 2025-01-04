#[cfg(feature = "HMUI+RoundedCornersDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoundedCornersDirection {
    #[default]
    All = 0i32,
    Down = 5i32,
    DownLeft = 6i32,
    DownRight = 4i32,
    Left = 7i32,
    Right = 3i32,
    Up = 1i32,
    UpLeft = 8i32,
    UpRight = 2i32,
}
#[cfg(feature = "HMUI+RoundedCornersDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::RoundedCornersDirection => "HMUI"
    ."RoundedCornersDirection"
);

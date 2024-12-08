#[cfg(feature = "OffsetDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetDirection {
    Down = 1i32,
    DownLeft = 6i32,
    DownRight = 7i32,
    Left = 2i32,
    None = 9i32,
    Right = 3i32,
    Up = 0i32,
    UpLeft = 4i32,
    UpRight = 5i32,
}
#[cfg(feature = "OffsetDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for OffsetDirection => ""."OffsetDirection"
);

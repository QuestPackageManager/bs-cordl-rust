#[cfg(feature = "System+ConsoleColor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleColor {
    Black = 0i32,
    Blue = 9i32,
    Cyan = 11i32,
    DarkBlue = 1i32,
    DarkCyan = 3i32,
    DarkGray = 8i32,
    DarkGreen = 2i32,
    DarkMagenta = 5i32,
    DarkRed = 4i32,
    DarkYellow = 6i32,
    Gray = 7i32,
    Green = 10i32,
    Magenta = 13i32,
    Red = 12i32,
    White = 15i32,
    Yellow = 14i32,
}
#[cfg(feature = "System+ConsoleColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleColor => "System"."ConsoleColor"
);

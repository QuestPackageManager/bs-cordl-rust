#[cfg(feature = "System+ConsoleModifiers")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleModifiers {
    Alt = 1i32,
    Control = 4i32,
    Shift = 2i32,
}
#[cfg(feature = "System+ConsoleModifiers")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleModifiers => "System"
    ."ConsoleModifiers"
);

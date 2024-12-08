#[cfg(feature = "System+ConsoleSpecialKey")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleSpecialKey {
    ControlBreak = 1i32,
    ControlC = 0i32,
}
#[cfg(feature = "System+ConsoleSpecialKey")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ConsoleSpecialKey => "System"
    ."ConsoleSpecialKey"
);

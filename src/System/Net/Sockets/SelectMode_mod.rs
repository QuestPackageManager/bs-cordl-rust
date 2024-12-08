#[cfg(feature = "System+Net+Sockets+SelectMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMode {
    SelectError = 2i32,
    SelectRead = 0i32,
    SelectWrite = 1i32,
}
#[cfg(feature = "System+Net+Sockets+SelectMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SelectMode =>
    "System.Net.Sockets"."SelectMode"
);

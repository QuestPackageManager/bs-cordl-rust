#[cfg(feature = "System+DTSubStringType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DTSubStringType {
    End = 3i32,
    Invalid = 1i32,
    Number = 2i32,
    Other = 4i32,
    Unknown = 0i32,
}
#[cfg(feature = "System+DTSubStringType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DTSubStringType => "System"
    ."DTSubStringType"
);

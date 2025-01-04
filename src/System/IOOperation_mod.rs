#[cfg(feature = "System+IOOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IOOperation {
    #[default]
    Read = 1i32,
    Write = 2i32,
}
#[cfg(feature = "System+IOOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IOOperation => "System"."IOOperation"
);

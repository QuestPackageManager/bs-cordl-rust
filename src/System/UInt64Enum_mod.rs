#[cfg(feature = "System+UInt64Enum")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UInt64Enum {}
#[cfg(feature = "System+UInt64Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UInt64Enum => "System"."UInt64Enum"
);

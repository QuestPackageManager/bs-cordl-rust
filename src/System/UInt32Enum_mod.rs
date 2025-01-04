#[cfg(feature = "System+UInt32Enum")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UInt32Enum {}
#[cfg(feature = "System+UInt32Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UInt32Enum => "System"."UInt32Enum"
);

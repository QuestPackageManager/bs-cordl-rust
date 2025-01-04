#[cfg(feature = "System+UInt16Enum")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UInt16Enum {}
#[cfg(feature = "System+UInt16Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UInt16Enum => "System"."UInt16Enum"
);

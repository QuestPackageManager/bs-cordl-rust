#[cfg(feature = "System+Int16Enum")]
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Int16Enum {}
#[cfg(feature = "System+Int16Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Int16Enum => "System"."Int16Enum"
);

#[cfg(feature = "System+Int32Enum")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Int32Enum {}
#[cfg(feature = "System+Int32Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Int32Enum => "System"."Int32Enum"
);

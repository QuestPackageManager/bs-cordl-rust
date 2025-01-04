#[cfg(feature = "System+Int64Enum")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Int64Enum {}
#[cfg(feature = "System+Int64Enum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Int64Enum => "System"."Int64Enum"
);

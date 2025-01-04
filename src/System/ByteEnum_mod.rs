#[cfg(feature = "System+ByteEnum")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteEnum {}
#[cfg(feature = "System+ByteEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ByteEnum => "System"."ByteEnum"
);

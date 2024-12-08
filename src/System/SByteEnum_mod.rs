#[cfg(feature = "System+SByteEnum")]
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SByteEnum {}
#[cfg(feature = "System+SByteEnum")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::SByteEnum => "System"."SByteEnum"
);

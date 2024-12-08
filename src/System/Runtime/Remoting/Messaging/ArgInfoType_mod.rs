#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfoType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgInfoType {
    In = 0u8,
    Out = 1u8,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+ArgInfoType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::ArgInfoType =>
    "System.Runtime.Remoting.Messaging"."ArgInfoType"
);

#[cfg(feature = "System+Runtime+Remoting+Messaging+CallType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CallType {
    #[default]
    BeginInvoke = 1i32,
    EndInvoke = 2i32,
    OneWay = 3i32,
    Sync = 0i32,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+CallType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Messaging::CallType
    => "System.Runtime.Remoting.Messaging"."CallType"
);

#[cfg(feature = "System+Runtime+Remoting+Lifetime+LeaseState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseState {
    Active = 2i32,
    Expired = 4i32,
    Initial = 1i32,
    Null = 0i32,
    Renewing = 3i32,
}
#[cfg(feature = "System+Runtime+Remoting+Lifetime+LeaseState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Lifetime::LeaseState
    => "System.Runtime.Remoting.Lifetime"."LeaseState"
);

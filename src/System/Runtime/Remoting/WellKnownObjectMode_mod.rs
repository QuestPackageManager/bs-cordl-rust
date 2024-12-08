#[cfg(feature = "System+Runtime+Remoting+WellKnownObjectMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellKnownObjectMode {
    SingleCall = 2i32,
    Singleton = 1i32,
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownObjectMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::WellKnownObjectMode
    => "System.Runtime.Remoting"."WellKnownObjectMode"
);

#[cfg(feature = "System+Runtime+Remoting+CustomErrorsModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CustomErrorsModes {
    #[default]
    Off = 1i32,
    On = 0i32,
    RemoteOnly = 2i32,
}
#[cfg(feature = "System+Runtime+Remoting+CustomErrorsModes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::CustomErrorsModes =>
    "System.Runtime.Remoting"."CustomErrorsModes"
);

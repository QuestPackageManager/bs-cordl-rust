#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClassInterfaceType {
    #[default]
    AutoDispatch = 1i32,
    AutoDual = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Runtime+InteropServices+ClassInterfaceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ClassInterfaceType =>
    "System.Runtime.InteropServices"."ClassInterfaceType"
);

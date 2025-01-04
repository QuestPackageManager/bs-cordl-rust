#[cfg(feature = "System+Runtime+InteropServices+ComInterfaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComInterfaceType {
    #[default]
    InterfaceIsDual = 0i32,
    InterfaceIsIDispatch = 2i32,
    InterfaceIsIInspectable = 3i32,
    InterfaceIsIUnknown = 1i32,
}
#[cfg(feature = "System+Runtime+InteropServices+ComInterfaceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ComInterfaceType =>
    "System.Runtime.InteropServices"."ComInterfaceType"
);

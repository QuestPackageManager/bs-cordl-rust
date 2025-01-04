#[cfg(feature = "System+Reflection+AssemblyContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssemblyContentType {
    #[default]
    Default = 0i32,
    WindowsRuntime = 1i32,
}
#[cfg(feature = "System+Reflection+AssemblyContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyContentType =>
    "System.Reflection"."AssemblyContentType"
);

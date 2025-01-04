#[cfg(feature = "System+Security+AccessControl+ResourceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceType {
    #[default]
    DSObject = 8i32,
    DSObjectAll = 9i32,
    FileObject = 1i32,
    KernelObject = 6i32,
    LMShare = 5i32,
    Printer = 3i32,
    ProviderDefined = 10i32,
    RegistryKey = 4i32,
    RegistryWow6432Key = 12i32,
    Service = 2i32,
    Unknown = 0i32,
    WindowObject = 7i32,
    WmiGuidObject = 11i32,
}
#[cfg(feature = "System+Security+AccessControl+ResourceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::ResourceType =>
    "System.Security.AccessControl"."ResourceType"
);

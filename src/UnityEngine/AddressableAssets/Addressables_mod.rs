#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
#[repr(C)]
#[derive(Debug)]
pub struct Addressables {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AddressableAssets::Addressables =>
    "UnityEngine.AddressableAssets"."Addressables"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::Addressables {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::Addressables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl crate::UnityEngine::AddressableAssets::Addressables {
    pub const kAddressablesRuntimeBuildLogPath: &'static str = "AddressablesRuntimeBuildLog";
    pub const kAddressablesRuntimeDataPath: &'static str = "AddressablesRuntimeDataPath";
    pub const k_AddressablesLogConditional: &'static str = "ADDRESSABLES_LOG_ALL";
    #[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
    pub type MergeMode = crate::UnityEngine::AddressableAssets::Addressables_MergeMode;
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Addressables {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Addressables_MergeMode {
    Intersection = 2i32,
    None = 0i32,
    Union = 1i32,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Addressables_MergeMode =>
    "UnityEngine.AddressableAssets"."Addressables/MergeMode"
);

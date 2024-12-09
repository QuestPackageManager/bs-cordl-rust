#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesRuntimeProperties {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties =>
    "UnityEngine.AddressableAssets.Initialization"."AddressablesRuntimeProperties"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

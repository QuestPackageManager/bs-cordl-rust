#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetReferenceGameObject {
    __cordl_parent: crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::UnityEngine::GameObject,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AssetReferenceGameObject =>
    "UnityEngine.AddressableAssets"."AssetReferenceGameObject"
);
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::AssetReferenceGameObject {
    type Target = crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::UnityEngine::GameObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::AssetReferenceGameObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
impl crate::UnityEngine::AddressableAssets::AssetReferenceGameObject {
    pub fn New(
        guid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (guid))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        guid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (guid))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceGameObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AssetReferenceGameObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

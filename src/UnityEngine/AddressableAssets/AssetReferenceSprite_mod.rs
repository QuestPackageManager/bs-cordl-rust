#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetReferenceSprite {
    __cordl_parent: crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::UnityEngine::Sprite,
    >,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AssetReferenceSprite =>
    "UnityEngine.AddressableAssets"."AssetReferenceSprite"
);
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::AssetReferenceSprite {
    type Target = crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::UnityEngine::Sprite,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::AssetReferenceSprite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
impl crate::UnityEngine::AddressableAssets::AssetReferenceSprite {
    pub fn New(
        guid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (guid))?;
        Ok(__cordl_object)
    }
    pub fn ValidateAsset(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateAsset", (path))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        guid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (guid))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceSprite")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AssetReferenceSprite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

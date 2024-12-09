#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetReferenceT_1<TObject: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::AddressableAssets::AssetReference,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AssetReferenceT_1 < TObject > =>
    "UnityEngine.AddressableAssets"."AssetReferenceT`1" < TObject >
);
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::AddressableAssets::AssetReferenceT_1<TObject> {
    type Target = crate::UnityEngine::AddressableAssets::AssetReference;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::AssetReferenceT_1<TObject> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> crate::UnityEngine::AddressableAssets::AssetReferenceT_1<TObject> {
    pub fn LoadAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAssetAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        guid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (guid))?;
        Ok(__cordl_object)
    }
    pub fn ValidateAsset_Object0(
        &mut self,
        obj: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateAsset", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateAsset_String1(
        &mut self,
        mainAssetPath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ValidateAsset", (mainAssetPath))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        guid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (guid))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AssetReferenceT_1")]
impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AssetReferenceT_1<TObject> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+AssetBundle")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundle {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+AssetBundle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssetBundle => "UnityEngine"
    ."AssetBundle"
);
#[cfg(feature = "UnityEngine+AssetBundle")]
impl std::ops::Deref for crate::UnityEngine::AssetBundle {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundle")]
impl std::ops::DerefMut for crate::UnityEngine::AssetBundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AssetBundle")]
impl crate::UnityEngine::AssetBundle {
    pub fn GetAllAssetNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetAllAssetNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllLoadedAssetBundles() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllLoadedAssetBundles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllLoadedAssetBundles_Native() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::AssetBundle>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::AssetBundle>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllLoadedAssetBundles_Native", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllScenePaths(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetAllScenePaths", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllAssetsAsync_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAllAssetsAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllAssetsAsync_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAllAssetsAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllAssetsAsync_Type2(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAllAssetsAsync", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAsync_Il2CppString0<T>(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAssetAsync", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAsync_Internal(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAssetAsync_Internal", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAsync_Type1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAssetAsync", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetWithSubAssetsAsync(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("LoadAssetWithSubAssetsAsync", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetWithSubAssetsAsync_Internal(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object
            .invoke("LoadAssetWithSubAssetsAsync_Internal", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("LoadAsset", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_Il2CppString1<T>(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("LoadAsset", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_Internal(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("LoadAsset_Internal", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_Type2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("LoadAsset", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFile(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFile", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFileAsync_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleCreateRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleCreateRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFileAsync", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFileAsync_Internal(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
        offset: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleCreateRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleCreateRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFileAsync_Internal", (path, crc, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFileAsync_u32_1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleCreateRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleCreateRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFileAsync", (path, crc))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFile_Internal(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crc: u32,
        offset: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFile_Internal", (path, crc, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unload(
        &mut self,
        unloadAllLoadedObjects: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", (unloadAllLoadedObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadAsync(
        &mut self,
        unloadAllLoadedObjects: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleUnloadOperation>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleUnloadOperation,
        > = __cordl_object.invoke("UnloadAsync", (unloadAllLoadedObjects))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AssetBundle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AssetBundle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

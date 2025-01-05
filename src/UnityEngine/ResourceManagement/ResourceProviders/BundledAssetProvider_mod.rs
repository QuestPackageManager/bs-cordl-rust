#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BundledAssetProvider {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider =>
    "UnityEngine.ResourceManagement.ResourceProviders"."BundledAssetProvider"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
    )]
    pub type InternalOp = crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Provide(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Provide", (provideHandle))?;
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BundledAssetProvider_InternalOp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AssetBundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    pub m_PreloadRequest: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_RequestOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_Result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_ProvideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub subObjectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp
    => "UnityEngine.ResourceManagement.ResourceProviders"
    ."BundledAssetProvider/InternalOp"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    pub fn ActionComplete(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActionComplete", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginAssetLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginAssetLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteOperation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetArrayResult", (allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetResult(
        &mut self,
        asset: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAssetResult", (asset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetSubObjectResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAssetSubObjectResult", (allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetListResult(
        &mut self,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Object>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetListResult", (allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBundleFromDependecies<T>(
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadBundleFromDependecies", (results))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProgressCallback(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ProgressCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (provideHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Start_b__7_0(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__7_0", (operation))?;
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+BundledAssetProvider+InternalOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::BundledAssetProvider_InternalOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

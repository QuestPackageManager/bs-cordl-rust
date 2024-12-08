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
    pub fn Provide(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Provide", (provideHandle))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
    pub m_AssetBundle: *mut crate::UnityEngine::AssetBundle,
    pub m_PreloadRequest: *mut crate::UnityEngine::AssetBundleRequest,
    pub m_RequestOperation: *mut crate::UnityEngine::AssetBundleRequest,
    pub m_Result: *mut crate::System::Object,
    pub m_ProvideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub subObjectName: *mut crate::System::String,
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
    type Target = crate::System::Object;
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
    pub fn GetAssetSubObjectResult(
        &mut self,
        allAssets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAssetSubObjectResult", (allAssets))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetResult(
        &mut self,
        asset: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAssetResult", (asset))?;
        Ok(__cordl_ret)
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetArrayResult(
        &mut self,
        allAssets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetArrayResult", (allAssets))?;
        Ok(__cordl_ret)
    }
    pub fn GetListResult(
        &mut self,
        allAssets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetListResult", (allAssets))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn _Start_b__7_0(
        &mut self,
        operation: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Start>b__7_0", (operation))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ActionComplete(
        &mut self,
        obj: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActionComplete", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn BeginAssetLoad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginAssetLoad", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProgressCallback(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ProgressCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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

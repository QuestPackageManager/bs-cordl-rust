#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct UpdateCatalogsOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >,
    pub m_Addressables: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AddressablesImpl,
    >,
    pub m_LocatorInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
        >,
    >,
    pub m_DepOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
    pub m_CleanCacheOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        bool,
    >,
    pub m_AutoCleanBundleCache: bool,
}
#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::UpdateCatalogsOperation =>
    "UnityEngine.AddressableAssets"."UpdateCatalogsOperation"
);
#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::UpdateCatalogsOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::UpdateCatalogsOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
impl crate::UnityEngine::AddressableAssets::UpdateCatalogsOperation {
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies(
        &mut self,
        dependencies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (dependencies))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aa))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCleanCacheCompleted(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCleanCacheCompleted", (handle, catalogs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        catalogIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        autoCleanBundleCache: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = __cordl_object.invoke("Start", (catalogIds, autoCleanBundleCache))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aa))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+UpdateCatalogsOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::UpdateCatalogsOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

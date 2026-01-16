#[cfg(
    feature = "cordl_class_UnityEngine+AddressableAssets+Initialization+InitializationOperation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InitializationOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >,
    pub m_rtdOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::Initialization::ResourceManagerRuntimeData,
        >,
    >,
    pub m_loadCatalogOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >,
    pub m_ProviderSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Addressables: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::AddressablesImpl,
    >,
    pub m_InitGroupOps: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+AddressableAssets+Initialization+InitializationOperation"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Initialization";
    const CLASS_NAME: &'static str = "InitializationOperation";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    pub fn CreateInitializationOperation(
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        playerSettingsLocation: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::AddressablesImpl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                            >,
                        >,
                        3usize,
                    >("CreateInitializationOperation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateInitializationOperation", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (aa, playerSettingsLocation, providerSuffix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("InvokeWaitForCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeWaitForCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalogInternal(
        &mut self,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        index: i32,
        locMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                                    >,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                            >,
                        >,
                        4usize,
                    >("LoadContentCatalogInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadContentCatalogInternal", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (catalogs, index, locMap, remoteHashLocation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalog_AddressablesImpl_IResourceLocation_Il2CppString_IResourceLocation0(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::AddressablesImpl,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                            >,
                        >,
                        4usize,
                    >("LoadContentCatalog")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadContentCatalog", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (addressables, loc, providerSuffix, remoteHashLocation),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalog_IResourceLocation_Il2CppString_IResourceLocation1(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                            >,
                        >,
                        3usize,
                    >("LoadContentCatalog")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadContentCatalog", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (loc, providerSuffix, remoteHashLocation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadOpComplete(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        locMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        index: i32,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("LoadOpComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadOpComplete", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (op, catalogs, locMap, index, remoteHashLocation),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadProvider(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        providerData: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::AddressablesImpl,
                            >,
                            crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("LoadProvider")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadProvider", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (addressables, providerData, providerSuffix))?
        };
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
    pub fn OnCatalogDataLoaded(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
            >,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::AddressablesImpl,
                            >,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                            >,
                        ),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                            >,
                        >,
                        4usize,
                    >("OnCatalogDataLoaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCatalogDataLoaded", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (addressables, op, providerSuffix, remoteHashLocation),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::AddressableAssets::AddressablesImpl,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (aa))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_DebugName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DebugName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_Progress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Progress", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+AddressableAssets+Initialization+InitializationOperation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

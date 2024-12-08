#[cfg(feature = "SteamPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamPlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::AdditionalContentModel,
    pub _steamLevelProductCollectionModel: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel,
    pub _entitlementsAppIds: *mut crate::System::Collections::Generic::HashSet_1<u32>,
    pub _dataIsValidTaskCompletionSource: *mut crate::System::Threading::Tasks::TaskCompletionSource_1<
        bool,
    >,
    pub _semaphoreSlim: *mut crate::System::Threading::SemaphoreSlim,
    pub _isDataValid: bool,
}
#[cfg(feature = "SteamPlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SteamPlatformAdditionalContentModel => ""
    ."SteamPlatformAdditionalContentModel"
);
#[cfg(feature = "SteamPlatformAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::SteamPlatformAdditionalContentModel {
    type Target = crate::GlobalNamespace::AdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamPlatformAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SteamPlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamPlatformAdditionalContentModel")]
impl crate::GlobalNamespace::SteamPlatformAdditionalContentModel {
    pub const _steamAppID: u32 = 0u32;
    #[cfg(
        feature = "SteamPlatformAdditionalContentModel+_GetLevelDataVersionInternalAsync_d__10"
    )]
    pub type _GetLevelDataVersionInternalAsync_d__10 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__GetLevelDataVersionInternalAsync_d__10;
    #[cfg(
        feature = "SteamPlatformAdditionalContentModel+_GetLevelEntitlementStatusInternalAsync_d__8"
    )]
    pub type _GetLevelEntitlementStatusInternalAsync_d__8 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__GetLevelEntitlementStatusInternalAsync_d__8;
    #[cfg(
        feature = "SteamPlatformAdditionalContentModel+_GetPackEntitlementStatusInternalAsync_d__9"
    )]
    pub type _GetPackEntitlementStatusInternalAsync_d__9 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__GetPackEntitlementStatusInternalAsync_d__9;
    #[cfg(
        feature = "SteamPlatformAdditionalContentModel+_GetRedirectedLevelPackProductData_d__11"
    )]
    pub type _GetRedirectedLevelPackProductData_d__11 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__GetRedirectedLevelPackProductData_d__11;
    #[cfg(feature = "SteamPlatformAdditionalContentModel+_IsDataValidAsync_d__12")]
    pub type _IsDataValidAsync_d__12 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__IsDataValidAsync_d__12;
    #[cfg(
        feature = "SteamPlatformAdditionalContentModel+_OpenLevelPackProductStoreAsync_d__14"
    )]
    pub type _OpenLevelPackProductStoreAsync_d__14 = crate::GlobalNamespace::SteamPlatformAdditionalContentModel__OpenLevelPackProductStoreAsync_d__14;
    pub fn CheckForNewEntitlementsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult = __cordl_object
            .invoke("CheckForNewEntitlementsAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelDataVersion(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapLevelDataVersion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelDataVersion = __cordl_object
            .invoke("GetLevelDataVersion", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelDataVersionInternalAsync(
        &mut self,
        levelId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::BeatmapLevelDataVersion,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::BeatmapLevelDataVersion,
        > = __cordl_object.invoke("GetLevelDataVersionInternalAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelEntitlementStatusInternalAsync(
        &mut self,
        levelId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementStatus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementStatus,
        > = __cordl_object
            .invoke(
                "GetLevelEntitlementStatusInternalAsync",
                (levelId, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelProductData(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData = __cordl_object
            .invoke("GetLevelProductData", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackEntitlementStatusInternalAsync(
        &mut self,
        packId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementStatus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::EntitlementStatus,
        > = __cordl_object
            .invoke(
                "GetPackEntitlementStatusInternalAsync",
                (packId, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetRedirectedLevelPackProductData(
        &mut self,
        packId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<u32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::System::Nullable_1<u32>,
        > = __cordl_object
            .invoke("GetRedirectedLevelPackProductData", (packId, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn HasLevelEntitlement(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLevelEntitlement", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn HasLevelPackEntitlement(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasLevelPackEntitlement", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn InvalidateDataInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateDataInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsDataValidAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("IsDataValidAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn IsPackBetterBuyThanLevelAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
        > = __cordl_object
            .invoke("IsPackBetterBuyThanLevelAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OpenBundleUrl(
        &mut self,
        bundleId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenBundleUrl", (bundleId))?;
        Ok(__cordl_ret)
    }
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::OpenProductStoreResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::OpenProductStoreResult,
        > = __cordl_object
            .invoke("OpenLevelPackProductStoreAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn OpenLevelProductStoreAsync(
        &mut self,
        levelId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::OpenProductStoreResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::OpenProductStoreResult,
        > = __cordl_object.invoke("OpenLevelProductStoreAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn OpenProductStore(
        &mut self,
        appId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenProductStore", (appId))?;
        Ok(__cordl_ret)
    }
    pub fn _HasLevelPackEntitlement_b__21_0(
        &mut self,
        levelProductData: *mut crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<HasLevelPackEntitlement>b__21_0", (levelProductData))?;
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
}
#[cfg(feature = "SteamPlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamPlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "OculusPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformAdditionalContentModel {
    __cordl_parent: AdditionalContentModel,
    pub _oculusLevelProductCollectionModel: *mut OculusLevelProductCollectionModel,
    pub _entitlementsSKU: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _semaphoreSlim: *mut crate::System::Threading::SemaphoreSlim,
    pub _isDataValid: bool,
}
#[cfg(feature = "OculusPlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusPlatformAdditionalContentModel => ""
    ."OculusPlatformAdditionalContentModel"
);
#[cfg(feature = "OculusPlatformAdditionalContentModel")]
impl std::ops::Deref for OculusPlatformAdditionalContentModel {
    type Target = AdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAdditionalContentModel")]
impl std::ops::DerefMut for OculusPlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAdditionalContentModel")]
impl OculusPlatformAdditionalContentModel {
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_OpenLevelProductStoreAsync_d__11"
    )]
    pub type _OpenLevelProductStoreAsync_d__11 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__OpenLevelProductStoreAsync_d__11;
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_GetRedirectedLevelPackProductData_d__5"
    )]
    pub type _GetRedirectedLevelPackProductData_d__5 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__GetRedirectedLevelPackProductData_d__5;
    #[cfg(feature = "OculusPlatformAdditionalContentModel+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel___c__DisplayClass13_0;
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_GetLevelDataVersionInternalAsync_d__9"
    )]
    pub type _GetLevelDataVersionInternalAsync_d__9 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__GetLevelDataVersionInternalAsync_d__9;
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_GetPackEntitlementStatusInternalAsync_d__8"
    )]
    pub type _GetPackEntitlementStatusInternalAsync_d__8 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__GetPackEntitlementStatusInternalAsync_d__8;
    #[cfg(feature = "OculusPlatformAdditionalContentModel+_IsDataValidAsync_d__10")]
    pub type _IsDataValidAsync_d__10 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__IsDataValidAsync_d__10;
    #[cfg(feature = "OculusPlatformAdditionalContentModel+__c__DisplayClass15_0")]
    pub type __c__DisplayClass15_0 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel___c__DisplayClass15_0;
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_GetLevelEntitlementStatusInternalAsync_d__7"
    )]
    pub type _GetLevelEntitlementStatusInternalAsync_d__7 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__GetLevelEntitlementStatusInternalAsync_d__7;
    #[cfg(
        feature = "OculusPlatformAdditionalContentModel+_OpenLevelPackProductStoreAsync_d__12"
    )]
    pub type _OpenLevelPackProductStoreAsync_d__12 = crate::GlobalNamespace::OculusPlatformAdditionalContentModel__OpenLevelPackProductStoreAsync_d__12;
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<OpenProductStoreResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            OpenProductStoreResult,
        > = __cordl_object
            .invoke("OpenLevelPackProductStoreAsync", (levelPackId, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelProductData(
        &mut self,
        GetLevelProductData: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData = __cordl_object
            .invoke("GetLevelProductData", (GetLevelProductData))?;
        Ok(__cordl_ret)
    }
    pub fn IsPackBetterBuyThanLevelAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<IsPackBetterBuyThanLevelResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            IsPackBetterBuyThanLevelResult,
        > = __cordl_object
            .invoke("IsPackBetterBuyThanLevelAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelEntitlementStatusInternalAsync(
        &mut self,
        levelId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<EntitlementStatus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        > = __cordl_object
            .invoke(
                "GetLevelEntitlementStatusInternalAsync",
                (levelId, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetRedirectedLevelPackProductData(
        &mut self,
        packId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object
            .invoke("GetRedirectedLevelPackProductData", (packId, cancellationToken))?;
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
    pub fn _HasLevelPackEntitlement_b__18_0(
        &mut self,
        levelProductData: *mut crate::GlobalNamespace::OculusLevelProductCollectionModel_LevelProductData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<HasLevelPackEntitlement>b__18_0", (levelProductData))?;
        Ok(__cordl_ret)
    }
    pub fn OpenLevelProductStoreAsync(
        &mut self,
        levelId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<OpenProductStoreResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            OpenProductStoreResult,
        > = __cordl_object
            .invoke("OpenLevelProductStoreAsync", (levelId, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForNewEntitlementsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
        > = __cordl_object.invoke("CheckForNewEntitlementsAsync", (cancellationToken))?;
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
    pub fn GetLevelDataVersion(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<BeatmapLevelDataVersion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapLevelDataVersion = __cordl_object
            .invoke("GetLevelDataVersion", (levelId))?;
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
    pub fn GetLevelDataVersionInternalAsync(
        &mut self,
        levelId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<BeatmapLevelDataVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            BeatmapLevelDataVersion,
        > = __cordl_object.invoke("GetLevelDataVersionInternalAsync", (levelId, token))?;
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
    pub fn GetPackEntitlementStatusInternalAsync(
        &mut self,
        packId: *mut crate::System::String,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<EntitlementStatus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        > = __cordl_object
            .invoke(
                "GetPackEntitlementStatusInternalAsync",
                (packId, cancellationToken),
            )?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusPlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType for OculusPlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

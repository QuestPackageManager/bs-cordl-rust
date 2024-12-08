#[cfg(feature = "MockPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformAdditionalContentModel {
    __cordl_parent: AdditionalContentModel,
    pub _levelsEntitlements: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MockPlatformEntitlement,
    >,
    pub _levelPacksEntitlements: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MockPlatformEntitlement,
    >,
    pub _packBetterBuyThanLevel: bool,
    pub randomMillisecondsResponseTime: crate::UnityEngine::RangeInt,
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlatformAdditionalContentModel => ""
    ."MockPlatformAdditionalContentModel"
);
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl std::ops::Deref for MockPlatformAdditionalContentModel {
    type Target = AdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl std::ops::DerefMut for MockPlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl MockPlatformAdditionalContentModel {
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_GetPackEntitlementStatusInternalAsync_d__7"
    )]
    pub type _GetPackEntitlementStatusInternalAsync_d__7 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__GetPackEntitlementStatusInternalAsync_d__7;
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_GetLevelEntitlementStatusInternalAsync_d__6"
    )]
    pub type _GetLevelEntitlementStatusInternalAsync_d__6 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__GetLevelEntitlementStatusInternalAsync_d__6;
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_IsPackBetterBuyThanLevelAsync_d__9"
    )]
    pub type _IsPackBetterBuyThanLevelAsync_d__9 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__IsPackBetterBuyThanLevelAsync_d__9;
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_OpenLevelPackProductStoreAsync_d__12"
    )]
    pub type _OpenLevelPackProductStoreAsync_d__12 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__OpenLevelPackProductStoreAsync_d__12;
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_OpenLevelProductStoreAsync_d__10"
    )]
    pub type _OpenLevelProductStoreAsync_d__10 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__OpenLevelProductStoreAsync_d__10;
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
    pub fn Wait(
        &mut self,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("Wait", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackEntitlementStatusInternalAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<EntitlementStatus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        > = __cordl_object
            .invoke("GetPackEntitlementStatusInternalAsync", (levelPackId, token))?;
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
    pub fn BuyLevel(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevel", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn OpenLevelProductStoreAsync(
        &mut self,
        levelId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<OpenProductStoreResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            OpenProductStoreResult,
        > = __cordl_object.invoke("OpenLevelProductStoreAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelEntitlementStatusInternalAsync(
        &mut self,
        levelId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<EntitlementStatus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            EntitlementStatus,
        > = __cordl_object
            .invoke("GetLevelEntitlementStatusInternalAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initialData: *mut MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialData))?;
        Ok(__cordl_ret)
    }
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<OpenProductStoreResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            OpenProductStoreResult,
        > = __cordl_object
            .invoke("OpenLevelPackProductStoreAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initialData: *mut MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType for MockPlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

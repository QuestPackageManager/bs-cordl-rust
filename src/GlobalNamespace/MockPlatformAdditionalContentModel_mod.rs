#[cfg(feature = "MockPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::AdditionalContentModel,
    pub _levelsEntitlements: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::MockPlatformEntitlement,
    >,
    pub _levelPacksEntitlements: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::MockPlatformEntitlement,
    >,
    pub _packBetterBuyThanLevel: bool,
    pub randomMillisecondsResponseTime: crate::UnityEngine::RangeInt,
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MockPlatformAdditionalContentModel => ""
    ."MockPlatformAdditionalContentModel"
);
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlatformAdditionalContentModel {
    type Target = crate::GlobalNamespace::AdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl crate::GlobalNamespace::MockPlatformAdditionalContentModel {
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_GetLevelEntitlementStatusInternalAsync_d__6"
    )]
    pub type _GetLevelEntitlementStatusInternalAsync_d__6 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__GetLevelEntitlementStatusInternalAsync_d__6;
    #[cfg(
        feature = "MockPlatformAdditionalContentModel+_GetPackEntitlementStatusInternalAsync_d__7"
    )]
    pub type _GetPackEntitlementStatusInternalAsync_d__7 = crate::GlobalNamespace::MockPlatformAdditionalContentModel__GetPackEntitlementStatusInternalAsync_d__7;
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
    pub fn BuyLevel(
        &mut self,
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuyLevel", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelDataVersionInternalAsync(
        &mut self,
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
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
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
        token: crate::System::Threading::CancellationToken,
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
            .invoke("GetLevelEntitlementStatusInternalAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackEntitlementStatusInternalAsync(
        &mut self,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
        token: crate::System::Threading::CancellationToken,
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
            .invoke("GetPackEntitlementStatusInternalAsync", (levelPackId, token))?;
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
    pub fn IsPackBetterBuyThanLevelAsync(
        &mut self,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New(
        initialData: *mut crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialData))?;
        Ok(__cordl_object)
    }
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: *mut quest_hook::libil2cpp::Il2CppString,
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
        levelId: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn _ctor(
        &mut self,
        initialData: *mut crate::GlobalNamespace::MockPlatformAdditionalContentModelInitialDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockPlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

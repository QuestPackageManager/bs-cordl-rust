#[cfg(feature = "SonyPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyPlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::AdditionalContentModel,
    pub _entitlementsLabels: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _semaphoreSlim: *mut crate::System::Threading::SemaphoreSlim,
    pub _isDataValid: bool,
    pub _sonyCommerceHelper: *mut crate::GlobalNamespace::ISonyCommerceHelper,
    pub _sonyLevelProductCollectionModel: *mut crate::GlobalNamespace::SonyLevelProductCollectionModel,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
}
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyPlatformAdditionalContentModel => ""
    ."SonyPlatformAdditionalContentModel"
);
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::SonyPlatformAdditionalContentModel {
    type Target = crate::GlobalNamespace::AdditionalContentModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyPlatformAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
impl crate::GlobalNamespace::SonyPlatformAdditionalContentModel {
    #[cfg(feature = "SonyPlatformAdditionalContentModel+_EnsureDataValidity_d__21")]
    pub type _EnsureDataValidity_d__21 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__EnsureDataValidity_d__21;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_GetLevelDataVersionInternalAsync_d__12"
    )]
    pub type _GetLevelDataVersionInternalAsync_d__12 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__GetLevelDataVersionInternalAsync_d__12;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_GetLevelEntitlementStatusInternalAsync_d__10"
    )]
    pub type _GetLevelEntitlementStatusInternalAsync_d__10 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__GetLevelEntitlementStatusInternalAsync_d__10;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_GetPackEntitlementStatusInternalAsync_d__11"
    )]
    pub type _GetPackEntitlementStatusInternalAsync_d__11 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__GetPackEntitlementStatusInternalAsync_d__11;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_GetRedirectedLevelPackProductData_d__17"
    )]
    pub type _GetRedirectedLevelPackProductData_d__17 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__GetRedirectedLevelPackProductData_d__17;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_IsPackBetterBuyThanLevelAsync_d__15"
    )]
    pub type _IsPackBetterBuyThanLevelAsync_d__15 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__IsPackBetterBuyThanLevelAsync_d__15;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_OpenLevelPackProductStoreAsync_d__14"
    )]
    pub type _OpenLevelPackProductStoreAsync_d__14 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__OpenLevelPackProductStoreAsync_d__14;
    #[cfg(
        feature = "SonyPlatformAdditionalContentModel+_OpenLevelProductStoreAsync_d__13"
    )]
    pub type _OpenLevelProductStoreAsync_d__13 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__OpenLevelProductStoreAsync_d__13;
    #[cfg(feature = "SonyPlatformAdditionalContentModel+_UpdateEntitlementsAsync_d__22")]
    pub type _UpdateEntitlementsAsync_d__22 = crate::GlobalNamespace::SonyPlatformAdditionalContentModel__UpdateEntitlementsAsync_d__22;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDataValidity(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object.invoke("EnsureDataValidity", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelDataVersion(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapLevelDataVersion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelDataVersion = __cordl_object
            .invoke("GetLevelDataVersion", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelDataVersionInternalAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::BeatmapLevelDataVersion,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::BeatmapLevelDataVersion,
            >,
        > = __cordl_object.invoke("GetLevelDataVersionInternalAsync", (levelId, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelEntitlementStatusInternalAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = __cordl_object
            .invoke(
                "GetLevelEntitlementStatusInternalAsync",
                (levelId, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPackEntitlementStatusInternalAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = __cordl_object
            .invoke(
                "GetPackEntitlementStatusInternalAsync",
                (levelPackId, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRedirectedLevelPackProductData(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object
            .invoke("GetRedirectedLevelPackProductData", (packId, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLevelEntitlement(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLevelEntitlement", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasLevelPackEntitlement(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasLevelPackEntitlement", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        vrPlatformHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVRPlatformHelper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (vrPlatformHelper))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateDataInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateDataInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPackBetterBuyThanLevelAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
            >,
        > = __cordl_object
            .invoke("IsPackBetterBuyThanLevelAsync", (levelPackId, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sonyCommerceHelper, sonyLevelProductCollectionModel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        > = __cordl_object
            .invoke("OpenLevelPackProductStoreAsync", (levelPackId, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenLevelProductStoreAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        > = __cordl_object
            .invoke("OpenLevelProductStoreAsync", (levelId, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenStore(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenStore", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateEntitlementsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
            >,
        > = __cordl_object.invoke("UpdateEntitlementsAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _HasLevelPackEntitlement_b__19_0(
        &mut self,
        levelProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<HasLevelPackEntitlement>b__19_0", (levelProductData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sonyCommerceHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ISonyCommerceHelper,
        >,
        sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyCommerceHelper, sonyLevelProductCollectionModel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyPlatformAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

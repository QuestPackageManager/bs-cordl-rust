#[cfg(feature = "SonyPlatformAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyPlatformAdditionalContentModel {
    __cordl_parent: crate::GlobalNamespace::AdditionalContentModel,
    pub _entitlementsLabels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _semaphoreSlim: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SemaphoreSlim,
    >,
    pub _isDataValid: bool,
    pub _sonyCommerceHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ISonyCommerceHelper,
    >,
    pub _sonyLevelProductCollectionModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SonyLevelProductCollectionModel,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
}
#[cfg(feature = "SonyPlatformAdditionalContentModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SonyPlatformAdditionalContentModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SonyPlatformAdditionalContentModel";
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureDataValidity(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
                1usize,
            >("EnsureDataValidity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "EnsureDataValidity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelDataVersion(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapLevelDataVersion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                crate::GlobalNamespace::BeatmapLevelDataVersion,
                1usize,
            >("GetLevelDataVersion")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "GetLevelDataVersion",
                    1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelDataVersion = unsafe {
            method.invoke_unchecked(self, (levelId))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::BeatmapLevelDataVersion,
                    >,
                >,
                2usize,
            >("GetLevelDataVersionInternalAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetLevelDataVersionInternalAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::BeatmapLevelDataVersion,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelId, token))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::EntitlementStatus,
                    >,
                >,
                2usize,
            >("GetLevelEntitlementStatusInternalAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetLevelEntitlementStatusInternalAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelId, cancellationToken))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::EntitlementStatus,
                    >,
                >,
                2usize,
            >("GetPackEntitlementStatusInternalAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetPackEntitlementStatusInternalAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelPackId, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRedirectedLevelPackProductData(
        &mut self,
        packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                2usize,
            >("GetRedirectedLevelPackProductData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "GetRedirectedLevelPackProductData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, (packId, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasLevelEntitlement(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("HasLevelEntitlement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "HasLevelEntitlement",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (levelId))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasLevelPackEntitlement(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("HasLevelPackEntitlement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "HasLevelPackEntitlement",
                    1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (levelPackId))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        vrPlatformHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVRPlatformHelper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IVRPlatformHelper>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (vrPlatformHelper))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvalidateDataInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("InvalidateDataInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "InvalidateDataInternal",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
                    >,
                >,
                2usize,
            >("IsPackBetterBuyThanLevelAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "IsPackBetterBuyThanLevelAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::IsPackBetterBuyThanLevelResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelPackId, cancellationToken))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::OpenProductStoreResult,
                    >,
                >,
                2usize,
            >("OpenLevelPackProductStoreAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "OpenLevelPackProductStoreAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelPackId, cancellationToken))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Threading::CancellationToken,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::OpenProductStoreResult,
                    >,
                >,
                2usize,
            >("OpenLevelProductStoreAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "OpenLevelProductStoreAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::OpenProductStoreResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelId, cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn OpenStore(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OpenStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "OpenStore", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uri))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Threading::CancellationToken),
                quest_hook::libil2cpp::Gc<
                    crate::System::Threading::Tasks::Task_1<
                        crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
                    >,
                >,
                1usize,
            >("UpdateEntitlementsAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateEntitlementsAsync",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::AdditionalContentModel_UpdateEntitlementsResult,
            >,
        > = unsafe { method.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn _GetRedirectedLevelPackProductData_g__GetProductLabel_17_0(
        levelPackProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelPackProductData,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("<GetRedirectedLevelPackProductData>g__GetProductLabel|17_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<GetRedirectedLevelPackProductData>g__GetProductLabel|17_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (levelPackProductData))? };
        Ok(__cordl_ret.into())
    }
    pub fn _HasLevelPackEntitlement_b__19_0(
        &mut self,
        levelProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
                >),
                bool,
                1usize,
            >("<HasLevelPackEntitlement>b__19_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<HasLevelPackEntitlement>b__19_0", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (levelProductData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _OpenLevelProductStoreAsync_g__GetProductLabelForProductBrowseDialog_13_0(
        levelProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SonyLevelProductCollectionModel_LevelProductData,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >(
                "<OpenLevelProductStoreAsync>g__GetProductLabelForProductBrowseDialog|13_0",
            )
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(),
                    "<OpenLevelProductStoreAsync>g__GetProductLabelForProductBrowseDialog|13_0",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (levelProductData))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SonyPlatformAdditionalContentModel as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ISonyCommerceHelper,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::SonyLevelProductCollectionModel,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SonyPlatformAdditionalContentModel as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (sonyCommerceHelper, sonyLevelProductCollectionModel),
                )?
        };
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

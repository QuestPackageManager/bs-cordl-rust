#[cfg(feature = "BeatmapLevelsEntitlementModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsEntitlementModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _additionalContentEntitlementModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAdditionalContentEntitlementModel,
    >,
    pub _alwaysOwnedBeatmapLevelIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _alwaysOwnedPacksIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelsEntitlementModel";
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
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    pub fn AddAlwaysOwnedPack(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddAlwaysOwnedPack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddAlwaysOwnedPack", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pack))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelDataVersionAsync(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
            >("GetLevelDataVersionAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLevelDataVersionAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::BeatmapLevelDataVersion,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelId, token)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelEntitlementStatusAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
            >("GetLevelEntitlementStatusAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLevelEntitlementStatusAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelId, token)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPackEntitlementStatusAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
            >("GetPackEntitlementStatusAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPackEntitlementStatusAsync", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = unsafe { method.invoke_unchecked(self, (levelPackId, token)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        additionalContentEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAdditionalContentEntitlementModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (additionalContentEntitlementModel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        additionalContentEntitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAdditionalContentEntitlementModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IAdditionalContentEntitlementModel,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (additionalContentEntitlementModel))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl AsRef<crate::GlobalNamespace::IEntitlementModel>
for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IEntitlementModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl AsMut<crate::GlobalNamespace::IEntitlementModel>
for crate::GlobalNamespace::BeatmapLevelsEntitlementModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IEntitlementModel {
        unsafe { std::mem::transmute(self) }
    }
}

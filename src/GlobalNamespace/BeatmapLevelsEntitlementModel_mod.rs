#[cfg(feature = "BeatmapLevelsEntitlementModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsEntitlementModel {
    __cordl_parent: crate::System::Object,
    pub _additionalContentEntitlementModel: *mut IAdditionalContentEntitlementModel,
    pub _alwaysOwnedBeatmapLevelIds: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _alwaysOwnedPacksIds: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelsEntitlementModel => ""
    ."BeatmapLevelsEntitlementModel"
);
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl std::ops::Deref for BeatmapLevelsEntitlementModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl std::ops::DerefMut for BeatmapLevelsEntitlementModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl BeatmapLevelsEntitlementModel {
    pub fn AddAlwaysOwnedPack(
        &mut self,
        pack: *mut PackDefinitionSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAlwaysOwnedPack", (pack))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelDataVersionAsync(
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
        > = __cordl_object.invoke("GetLevelDataVersionAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelEntitlementStatusAsync(
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
        > = __cordl_object.invoke("GetLevelEntitlementStatusAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackEntitlementStatusAsync(
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
            .invoke("GetPackEntitlementStatusAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        additionalContentEntitlementModel: *mut IAdditionalContentEntitlementModel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (additionalContentEntitlementModel))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        additionalContentEntitlementModel: *mut IAdditionalContentEntitlementModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (additionalContentEntitlementModel))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelsEntitlementModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

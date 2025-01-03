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
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub _alwaysOwnedPacksIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
}
#[cfg(feature = "BeatmapLevelsEntitlementModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelsEntitlementModel
    => ""."BeatmapLevelsEntitlementModel"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAlwaysOwnedPack", (pack))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::BeatmapLevelDataVersion,
            >,
        > = __cordl_object.invoke("GetLevelDataVersionAsync", (levelId, token))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = __cordl_object.invoke("GetLevelEntitlementStatusAsync", (levelId, token))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::EntitlementStatus,
            >,
        > = __cordl_object
            .invoke("GetPackEntitlementStatusAsync", (levelPackId, token))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (additionalContentEntitlementModel))?;
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

#[cfg(feature = "IAdditionalContentEntitlementModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IAdditionalContentEntitlementModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAdditionalContentEntitlementModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IAdditionalContentEntitlementModel => ""
    ."IAdditionalContentEntitlementModel"
);
#[cfg(feature = "IAdditionalContentEntitlementModel")]
impl std::ops::Deref for crate::GlobalNamespace::IAdditionalContentEntitlementModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentEntitlementModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::IAdditionalContentEntitlementModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentEntitlementModel")]
impl crate::GlobalNamespace::IAdditionalContentEntitlementModel {
    pub fn GetLevelDataVersionAsync(
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
        > = __cordl_object.invoke("GetLevelDataVersionAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelEntitlementStatusAsync(
        &mut self,
        levelId: *mut crate::System::String,
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
        > = __cordl_object.invoke("GetLevelEntitlementStatusAsync", (levelId, token))?;
        Ok(__cordl_ret)
    }
    pub fn GetPackEntitlementStatusAsync(
        &mut self,
        levelPackId: *mut crate::System::String,
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
            .invoke("GetPackEntitlementStatusAsync", (levelPackId, token))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IAdditionalContentEntitlementModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IAdditionalContentEntitlementModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct ISaveDataHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::ISaveDataHandler =>
    "BGLib.SaveDataCore"."ISaveDataHandler"
);
#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
impl std::ops::Deref for crate::BGLib::SaveDataCore::ISaveDataHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
impl std::ops::DerefMut for crate::BGLib::SaveDataCore::ISaveDataHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
impl crate::BGLib::SaveDataCore::ISaveDataHandler {
    pub fn GetState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::SaveDataCore::LoaderState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::SaveDataCore::LoaderState = __cordl_object
            .invoke("GetState", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetChangesAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("ResetChangesAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveAsync(
        &mut self,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::BGLib::SaveDataCore::SaveDataResult,
        > = __cordl_object.invoke("SaveAsync", (force))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_instance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BGLib::SaveDataCore::VersionableSaveData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BGLib::SaveDataCore::VersionableSaveData = __cordl_object
            .invoke("get_instance", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISaveDataHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::SaveDataCore::ISaveDataHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

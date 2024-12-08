#[cfg(feature = "IAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IAdditionalContentModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IAdditionalContentModel => ""."IAdditionalContentModel"
);
#[cfg(feature = "IAdditionalContentModel")]
impl std::ops::Deref for IAdditionalContentModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl std::ops::DerefMut for IAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl IAdditionalContentModel {
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
    pub fn remove_didInvalidateDataEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInvalidateDataEvent", (value))?;
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
    pub fn add_didInvalidateDataEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInvalidateDataEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType for IAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "IAdditionalContentModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IAdditionalContentModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IAdditionalContentModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IAdditionalContentModel => ""
    ."IAdditionalContentModel"
);
#[cfg(feature = "IAdditionalContentModel")]
impl std::ops::Deref for crate::GlobalNamespace::IAdditionalContentModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::IAdditionalContentModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl crate::GlobalNamespace::IAdditionalContentModel {
    pub fn IsPackBetterBuyThanLevelAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
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
            .invoke("IsPackBetterBuyThanLevelAsync", (levelPackId, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenLevelPackProductStoreAsync(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
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
            .invoke("OpenLevelPackProductStoreAsync", (levelPackId, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenLevelProductStoreAsync(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Threading::CancellationToken,
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
        > = __cordl_object.invoke("OpenLevelProductStoreAsync", (levelId, token))?;
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
    pub fn add_didInvalidateDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInvalidateDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn remove_didInvalidateDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInvalidateDataEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IAdditionalContentModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IAdditionalContentModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

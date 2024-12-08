#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct ISerializableSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::SaveDataCore::ISerializableSaveData =>
    "BGLib.SaveDataCore"."ISerializableSaveData"
);
#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
impl std::ops::Deref for crate::BGLib::SaveDataCore::ISerializableSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
impl std::ops::DerefMut for crate::BGLib::SaveDataCore::ISerializableSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
impl crate::BGLib::SaveDataCore::ISerializableSaveData {
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDirty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGLib+SaveDataCore+ISerializableSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::SaveDataCore::ISerializableSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

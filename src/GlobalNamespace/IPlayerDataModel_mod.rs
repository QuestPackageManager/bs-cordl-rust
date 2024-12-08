#[cfg(feature = "IPlayerDataModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlayerDataModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IPlayerDataModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IPlayerDataModel => ""
    ."IPlayerDataModel"
);
#[cfg(feature = "IPlayerDataModel")]
impl std::ops::Deref for crate::GlobalNamespace::IPlayerDataModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlayerDataModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::IPlayerDataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlayerDataModel")]
impl crate::GlobalNamespace::IPlayerDataModel {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_playerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerData = __cordl_object
            .invoke("get_playerData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IPlayerDataModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IPlayerDataModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

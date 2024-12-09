#[cfg(feature = "PlayerSaveDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataConvertor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlayerSaveDataConvertor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSaveDataConvertor => ""
    ."PlayerSaveDataConvertor"
);
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSaveDataConvertor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl crate::GlobalNamespace::PlayerSaveDataConvertor {}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSaveDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

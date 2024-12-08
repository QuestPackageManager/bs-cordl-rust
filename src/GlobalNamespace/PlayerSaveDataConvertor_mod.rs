#[cfg(feature = "PlayerSaveDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSaveDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PlayerSaveDataConvertor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerSaveDataConvertor => ""."PlayerSaveDataConvertor"
);
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::Deref for PlayerSaveDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl std::ops::DerefMut for PlayerSaveDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl PlayerSaveDataConvertor {}
#[cfg(feature = "PlayerSaveDataConvertor")]
impl quest_hook::libil2cpp::ObjectType for PlayerSaveDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

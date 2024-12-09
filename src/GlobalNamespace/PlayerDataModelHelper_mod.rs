#[cfg(feature = "PlayerDataModelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerDataModelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlayerDataModelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerDataModelHelper => ""
    ."PlayerDataModelHelper"
);
#[cfg(feature = "PlayerDataModelHelper")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerDataModelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerDataModelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataModelHelper")]
impl crate::GlobalNamespace::PlayerDataModelHelper {}
#[cfg(feature = "PlayerDataModelHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerDataModelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

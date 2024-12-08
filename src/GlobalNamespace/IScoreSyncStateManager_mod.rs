#[cfg(feature = "IScoreSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IScoreSyncStateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IScoreSyncStateManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IScoreSyncStateManager => ""
    ."IScoreSyncStateManager"
);
#[cfg(feature = "IScoreSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::IScoreSyncStateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IScoreSyncStateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl crate::GlobalNamespace::IScoreSyncStateManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IScoreSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IScoreSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

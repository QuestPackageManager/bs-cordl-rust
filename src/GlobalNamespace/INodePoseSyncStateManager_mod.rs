#[cfg(feature = "INodePoseSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct INodePoseSyncStateManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INodePoseSyncStateManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INodePoseSyncStateManager => ""
    ."INodePoseSyncStateManager"
);
#[cfg(feature = "INodePoseSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::INodePoseSyncStateManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::INodePoseSyncStateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
impl crate::GlobalNamespace::INodePoseSyncStateManager {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "INodePoseSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INodePoseSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

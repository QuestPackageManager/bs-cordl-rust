#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
#[repr(C)]
#[derive(Debug)]
pub struct IEventMerger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::IEventMerger
    => "UnityEngine.InputSystem.LowLevel"."IEventMerger"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
impl crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
    pub fn MergeForward(
        &mut self,
        currentEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        nextEventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MergeForward", (currentEventPtr, nextEventPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IEventMerger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::IEventMerger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

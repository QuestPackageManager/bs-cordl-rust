#[cfg(feature = "UnityEngine+FrameTimingManager")]
#[repr(C)]
#[derive(Debug)]
pub struct FrameTimingManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FrameTimingManager => "UnityEngine"
    ."FrameTimingManager"
);
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl std::ops::Deref for crate::UnityEngine::FrameTimingManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl std::ops::DerefMut for crate::UnityEngine::FrameTimingManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl crate::UnityEngine::FrameTimingManager {}
#[cfg(feature = "UnityEngine+FrameTimingManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::FrameTimingManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

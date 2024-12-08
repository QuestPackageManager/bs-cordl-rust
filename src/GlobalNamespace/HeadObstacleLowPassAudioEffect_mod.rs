#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct HeadObstacleLowPassAudioEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerHeadAndObstacleInteraction: *mut PlayerHeadAndObstacleInteraction,
    pub _mainAudioEffects: *mut MainAudioEffects,
    pub _headWasInObstacle: bool,
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HeadObstacleLowPassAudioEffect => ""
    ."HeadObstacleLowPassAudioEffect"
);
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl std::ops::Deref for HeadObstacleLowPassAudioEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl std::ops::DerefMut for HeadObstacleLowPassAudioEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl HeadObstacleLowPassAudioEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl quest_hook::libil2cpp::ObjectType for HeadObstacleLowPassAudioEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
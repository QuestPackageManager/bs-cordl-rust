#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct HeadObstacleLowPassAudioEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _playerHeadAndObstacleInteraction: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerHeadAndObstacleInteraction,
    >,
    pub _mainAudioEffects: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MainAudioEffects,
    >,
    pub _headWasInObstacle: bool,
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HeadObstacleLowPassAudioEffect
    => ""."HeadObstacleLowPassAudioEffect"
);
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl std::ops::Deref for crate::GlobalNamespace::HeadObstacleLowPassAudioEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::HeadObstacleLowPassAudioEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl crate::GlobalNamespace::HeadObstacleLowPassAudioEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HeadObstacleLowPassAudioEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HeadObstacleLowPassAudioEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

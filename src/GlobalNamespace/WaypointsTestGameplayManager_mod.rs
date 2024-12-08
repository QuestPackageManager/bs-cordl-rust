#[cfg(feature = "WaypointsTestGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointsTestGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _outerCapsuleStart: crate::UnityEngine::Vector3,
    pub _outerCapsuleEnd: crate::UnityEngine::Vector3,
    pub _outerCapsuleRadius: f32,
    pub _innerSphereOffset: crate::UnityEngine::Vector3,
    pub _innerSphereRadius: f32,
    pub _layersToColliderWith: crate::UnityEngine::LayerMask,
    pub _characterSpawnController: *mut BTSCharacterSpawnController,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _standardSceneSetupData: *mut StandardGameplaySceneSetupData,
    pub _levelSceneSetupData: *mut GameplayCoreSceneSetupData,
    pub _target: *mut crate::UnityEngine::Transform,
    pub _firstPosSaved: bool,
    pub _speedSaved: bool,
    pub _lastFramePos: crate::UnityEngine::Vector3,
    pub _lastFrameSpeed: f32,
    pub _biggestFrameSpeed: f32,
    pub _biggestFrameSpeedSongTime: f32,
    pub _biggestAcceleration: f32,
    pub _biggestAccelerationSongTime: f32,
}
#[cfg(feature = "WaypointsTestGameplayManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for WaypointsTestGameplayManager => ""
    ."WaypointsTestGameplayManager"
);
#[cfg(feature = "WaypointsTestGameplayManager")]
impl std::ops::Deref for WaypointsTestGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestGameplayManager")]
impl std::ops::DerefMut for WaypointsTestGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointsTestGameplayManager")]
impl WaypointsTestGameplayManager {
    pub fn GetBar(
        &mut self,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetBar", (songTime))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDrawGizmos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrawGizmos", ())?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "WaypointsTestGameplayManager")]
impl quest_hook::libil2cpp::ObjectType for WaypointsTestGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

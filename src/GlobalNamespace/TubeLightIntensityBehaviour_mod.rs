#[cfg(feature = "TubeLightIntensityBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeLightIntensityBehaviour {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub _noPredefinedStartValue: bool,
    pub _startLightIntensity: f32,
    pub _startLaserIntensity: f32,
    pub _endLightIntensity: f32,
    pub _endLaserIntensity: f32,
    pub _disableWhenFinished: bool,
    pub _blend: f32,
    pub _initialized: bool,
    pub _originalLightIntensity: f32,
    pub _originalLaserIntensity: f32,
    pub _tubeLights: *mut quest_hook::libil2cpp::Il2CppArray<*mut TubeBloomPrePassLight>,
    pub _directionalLights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut DirectionalLight,
    >,
    pub _started: bool,
    pub _finished: bool,
    pub _firstFrameLightIntensity: f32,
    pub _firstFrameLaserIntensity: f32,
}
#[cfg(feature = "TubeLightIntensityBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TubeLightIntensityBehaviour => ""
    ."TubeLightIntensityBehaviour"
);
#[cfg(feature = "TubeLightIntensityBehaviour")]
impl std::ops::Deref for TubeLightIntensityBehaviour {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightIntensityBehaviour")]
impl std::ops::DerefMut for TubeLightIntensityBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightIntensityBehaviour")]
impl TubeLightIntensityBehaviour {
    pub fn EnableObjects(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableObjects", (on))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnPlayableDestroy(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPlayableDestroy", (playable))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFrame", (playable, info, playerData))?;
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
#[cfg(feature = "TubeLightIntensityBehaviour")]
impl quest_hook::libil2cpp::ObjectType for TubeLightIntensityBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
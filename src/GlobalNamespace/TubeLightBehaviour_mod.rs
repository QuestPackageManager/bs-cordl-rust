#[cfg(feature = "TubeLightBehaviour")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeLightBehaviour {
    __cordl_parent: crate::UnityEngine::Playables::PlayableBehaviour,
    pub _noPredefinedStartValue: bool,
    pub startColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub endColor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub blend: f32,
    pub _initialized: bool,
    pub _originalColor: crate::UnityEngine::Color,
    pub _tubeLights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::TubeBloomPrePassLight,
        >,
    >,
    pub _directionalLights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::DirectionalLight>,
    >,
    pub started: bool,
    pub _firstFrameColor: crate::UnityEngine::Color,
}
#[cfg(feature = "TubeLightBehaviour")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TubeLightBehaviour => ""
    ."TubeLightBehaviour"
);
#[cfg(feature = "TubeLightBehaviour")]
impl std::ops::Deref for crate::GlobalNamespace::TubeLightBehaviour {
    type Target = crate::UnityEngine::Playables::PlayableBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightBehaviour")]
impl std::ops::DerefMut for crate::GlobalNamespace::TubeLightBehaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeLightBehaviour")]
impl crate::GlobalNamespace::TubeLightBehaviour {
    #[cfg(feature = "TubeLightBehaviour+ParameterType")]
    pub type ParameterType = crate::GlobalNamespace::TubeLightBehaviour_ParameterType;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessFrame(
        &mut self,
        playable: crate::UnityEngine::Playables::Playable,
        info: crate::UnityEngine::Playables::FrameData,
        playerData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFrame", (playable, info, playerData))?;
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
#[cfg(feature = "TubeLightBehaviour")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TubeLightBehaviour {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TubeLightBehaviour+ParameterType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TubeLightBehaviour_ParameterType {
    #[default]
    References = 1i32,
    Values = 0i32,
}
#[cfg(feature = "TubeLightBehaviour+ParameterType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TubeLightBehaviour_ParameterType => ""
    ."TubeLightBehaviour/ParameterType"
);

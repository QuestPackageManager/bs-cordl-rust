#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ArcAndObstacleHapticEffectManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _hapticPreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    pub _obstacleSaberSparkleEffectManager: *mut crate::GlobalNamespace::ObstacleSaberSparkleEffectManager,
    pub _sliderInteractionManagers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::SliderInteractionManager,
    >,
    pub _saberManager: *mut crate::GlobalNamespace::SaberManager,
    pub _hapticFeedbackManager: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub _leftHandEffectState: *mut crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
    pub _rightHandEffectState: *mut crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ArcAndObstacleHapticEffectManager => ""
    ."ArcAndObstacleHapticEffectManager"
);
#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
impl std::ops::Deref for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
impl crate::GlobalNamespace::ArcAndObstacleHapticEffectManager {
    #[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
    pub type EffectState = crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState;
    pub fn GetState_ColorType1(
        &mut self,
        colorType: crate::GlobalNamespace::ColorType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
        > = __cordl_object.invoke("GetState", (colorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetState_SaberType0(
        &mut self,
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
        > = __cordl_object.invoke("GetState", (saberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderInteractionManagerAllSliderWereRemovedFromActiveSliders(
        &mut self,
        sliderInteractionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderInteractionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSliderInteractionManagerAllSliderWereRemovedFromActiveSliders",
                (sliderInteractionManager),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSliderInteractionManagerSliderWasAddedToActiveSliders(
        &mut self,
        sliderInteractionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderInteractionManager,
        >,
        sliderInteractionParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSliderInteractionManagerSliderWasAddedToActiveSliders",
                (sliderInteractionManager, sliderInteractionParam),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSparkleEffectDidEnd(
        &mut self,
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSparkleEffectDidEnd", (saberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSparkleEffectDidStart(
        &mut self,
        saberType: crate::GlobalNamespace::SaberType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSparkleEffectDidStart", (saberType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryDisableThis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryDisableThis", ())?;
        Ok(__cordl_ret.into())
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
    pub fn UpdateHaptic(
        &mut self,
        state: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState,
        >,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHaptic", (state, node))?;
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
#[cfg(feature = "ArcAndObstacleHapticEffectManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
#[repr(C)]
#[derive(Debug)]
pub struct ArcAndObstacleHapticEffectManager_EffectState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub colorType: crate::GlobalNamespace::ColorType,
    pub sliderInteractionManager: *mut crate::GlobalNamespace::SliderInteractionManager,
    pub isInTheObstacle: bool,
    pub isSliderActive: bool,
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState => ""
    ."ArcAndObstacleHapticEffectManager/EffectState"
);
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
impl std::ops::Deref
for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
impl crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_canBeActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canBeActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ArcAndObstacleHapticEffectManager+EffectState")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ArcAndObstacleHapticEffectManager_EffectState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

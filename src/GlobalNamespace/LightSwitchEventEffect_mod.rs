#[cfg(feature = "LightSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightColor0: *mut ColorSO,
    pub _lightColor1: *mut ColorSO,
    pub _highlightColor0: *mut ColorSO,
    pub _highlightColor1: *mut ColorSO,
    pub _lightColor0Boost: *mut ColorSO,
    pub _lightColor1Boost: *mut ColorSO,
    pub _highlightColor0Boost: *mut ColorSO,
    pub _highlightColor1Boost: *mut ColorSO,
    pub _offColorIntensity: f32,
    pub _lightOnStart: bool,
    pub _lightsID: i32,
    pub _event: BasicBeatmapEventType,
    pub _lightManager: *mut LightWithIdManager,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _colorManager: *mut ColorManager,
    pub _colorTween: *mut crate::Tweening::ColorTween,
    pub _alternativeFromColor: crate::UnityEngine::Color,
    pub _alternativeToColor: crate::UnityEngine::Color,
    pub _usingBoostColors: bool,
    pub _colorChangeBeatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _colorBoostBeatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "LightSwitchEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightSwitchEventEffect => ""."LightSwitchEventEffect"
);
#[cfg(feature = "LightSwitchEventEffect")]
impl std::ops::Deref for LightSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl std::ops::DerefMut for LightSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl LightSwitchEventEffect {
    pub const kFlashAndFadeDuration: f32 = 1.5f32;
    pub const kHighlightDuration: f32 = 0.6f32;
    pub fn CheckNextEventForFade(
        &mut self,
        basicBeatmapEventData: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNextEventForFade", (basicBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn GetHighlightColor(
        &mut self,
        beatmapEventValue: i32,
        colorBoost: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetHighlightColor", (beatmapEventValue, colorBoost))?;
        Ok(__cordl_ret)
    }
    pub fn GetNormalColor(
        &mut self,
        beatmapEventValue: i32,
        colorBoost: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetNormalColor", (beatmapEventValue, colorBoost))?;
        Ok(__cordl_ret)
    }
    pub fn HandleColorBoostBeatmapEvent(
        &mut self,
        eventData: *mut ColorBoostBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorBoostBeatmapEvent", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn HandleColorChangeBeatmapEvent(
        &mut self,
        basicBeatmapEventData: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorChangeBeatmapEvent", (basicBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn SetColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (color))?;
        Ok(__cordl_ret)
    }
    pub fn SetupTweenAndSaveOtherColors(
        &mut self,
        fromColor: crate::UnityEngine::Color,
        toColor: crate::UnityEngine::Color,
        alternativeFromColor: crate::UnityEngine::Color,
        alternativeToColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupTweenAndSaveOtherColors",
                (fromColor, toColor, alternativeFromColor, alternativeToColor),
            )?;
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
    pub fn get_eventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BasicBeatmapEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BasicBeatmapEventType = __cordl_object
            .invoke("get_eventType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightsId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightsId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType for LightSwitchEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

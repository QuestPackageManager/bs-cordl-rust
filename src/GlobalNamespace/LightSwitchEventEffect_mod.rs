#[cfg(feature = "LightSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightColor0: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _lightColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightColor0: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightColor1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _lightColor0Boost: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _lightColor1Boost: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    pub _highlightColor0Boost: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
    pub _highlightColor1Boost: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
    pub _offColorIntensity: f32,
    pub _lightOnStart: bool,
    pub _lightsID: i32,
    pub _event: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _lightManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightWithIdManager,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _colorTween: quest_hook::libil2cpp::Gc<crate::Tweening::ColorTween>,
    pub _alternativeFromColor: crate::UnityEngine::Color,
    pub _alternativeToColor: crate::UnityEngine::Color,
    pub _usingBoostColors: bool,
    pub _colorChangeBeatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _colorBoostBeatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "LightSwitchEventEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightSwitchEventEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightSwitchEventEffect";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::LightSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl crate::GlobalNamespace::LightSwitchEventEffect {
    pub const kFlashAndFadeDuration: f32 = 1.5f32;
    pub const kHighlightDuration: f32 = 0.6f32;
    pub fn CheckNextEventForFade(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckNextEventForFade", (basicBeatmapEventData))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorBoostBeatmapEvent(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorBoostBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorBoostBeatmapEvent", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorChangeBeatmapEvent(
        &mut self,
        basicBeatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BasicBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorChangeBeatmapEvent", (basicBeatmapEventData))?;
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
    pub fn SetColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (color))?;
        Ok(__cordl_ret.into())
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
    pub fn get_eventType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BasicBeatmapEventType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BasicBeatmapEventType = __cordl_object
            .invoke("get_eventType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightsId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lightsId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightSwitchEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

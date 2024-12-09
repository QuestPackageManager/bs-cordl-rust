#[cfg(feature = "LightColorGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _lightId: i32,
    pub _lightManager: *mut crate::GlobalNamespace::LightWithIdManager,
    pub _tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _bpmController: *mut crate::GlobalNamespace::IBpmController,
    pub _floatTween: *mut crate::Tweening::FloatTween,
    pub _lightColorBeatmapEventCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _fromColor: crate::UnityEngine::Color,
    pub _toColor: crate::UnityEngine::Color,
    pub _alternativeFromColor: crate::UnityEngine::Color,
    pub _alternativeToColor: crate::UnityEngine::Color,
    pub _fromStrobeFrequency: f32,
    pub _toStrobeFrequency: f32,
    pub _fromStrobeBrightness: f32,
    pub _toStrobeBrightness: f32,
    pub _strobeFade: bool,
    pub _usingBoostColors: bool,
}
#[cfg(feature = "LightColorGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightColorGroupEffect => ""
    ."LightColorGroupEffect"
);
#[cfg(feature = "LightColorGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffect")]
impl crate::GlobalNamespace::LightColorGroupEffect {
    #[cfg(feature = "LightColorGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::LightColorGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetColor(
        &mut self,
        colorType: crate::GlobalNamespace::EnvironmentColorType,
        colorBoost: bool,
        brightness: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColor", (colorType, colorBoost, brightness))?;
        Ok(__cordl_ret)
    }
    pub fn HandleColorChangeBeatmapEvent(
        &mut self,
        currentEventData: *mut crate::GlobalNamespace::LightColorBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorChangeBeatmapEvent", (currentEventData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::LightColorGroupEffect_InitData,
        lightManager: *mut crate::GlobalNamespace::LightWithIdManager,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        colorManager: *mut crate::GlobalNamespace::ColorManager,
        beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
        bpmController: *mut crate::GlobalNamespace::IBpmController,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    initData,
                    lightManager,
                    tweeningManager,
                    colorManager,
                    beatmapCallbacksController,
                    bpmController,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetColor(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (t))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        fromColor: crate::UnityEngine::Color,
        toColor: crate::UnityEngine::Color,
        alternativeFromColor: crate::UnityEngine::Color,
        alternativeToColor: crate::UnityEngine::Color,
        fromStrobeBeatFrequency: f32,
        toStrobeBeatFrequency: f32,
        fromStrobeBrightness: f32,
        toStrobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    fromColor,
                    toColor,
                    alternativeFromColor,
                    alternativeToColor,
                    fromStrobeBeatFrequency,
                    toStrobeBeatFrequency,
                    fromStrobeBrightness,
                    toStrobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UseBoostColors(
        &mut self,
        useBoostColors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UseBoostColors", (useBoostColors))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::LightColorGroupEffect_InitData,
        lightManager: *mut crate::GlobalNamespace::LightWithIdManager,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        colorManager: *mut crate::GlobalNamespace::ColorManager,
        beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
        bpmController: *mut crate::GlobalNamespace::IBpmController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    initData,
                    lightManager,
                    tweeningManager,
                    colorManager,
                    beatmapCallbacksController,
                    bpmController,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightColorGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightColorGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightColorGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorGroupEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub groupId: i32,
    pub elementId: i32,
    pub lightId: i32,
}
#[cfg(feature = "LightColorGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightColorGroupEffect_InitData
    => ""."LightColorGroupEffect/InitData"
);
#[cfg(feature = "LightColorGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupEffect+InitData")]
impl crate::GlobalNamespace::LightColorGroupEffect_InitData {
    pub fn New(
        groupId: i32,
        elementId: i32,
        lightId: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, elementId, lightId))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        lightId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, elementId, lightId))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightColorGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightColorGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

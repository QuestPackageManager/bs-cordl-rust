#[cfg(feature = "BufferedLightColorGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedLightColorGroupEffect {
    __cordl_parent: crate::System::Object,
    pub _colorManager: *mut ColorManager,
    pub _materialPropertyBlockController: *mut MaterialPropertyBlockController,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _colorBoostBeatmapDataCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _lightColorBeatmapEventCallbackWrappers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapDataCallbackWrapper,
    >,
    pub _lastIndex: i32,
    pub _timesBuffer: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _colorsBuffer: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Vector4,
    >,
    pub _elementIdsBuffer: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _useBoostColors: bool,
    pub _didReceiveEventThisFrame: bool,
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BufferedLightColorGroupEffect => ""
    ."BufferedLightColorGroupEffect"
);
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl std::ops::Deref for BufferedLightColorGroupEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl std::ops::DerefMut for BufferedLightColorGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl BufferedLightColorGroupEffect {
    pub const kBufferSize: i32 = 24i32;
    #[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData;
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData,
        colorManager: *mut ColorManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, colorManager, beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
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
    pub fn HandleColorChangeBeatmapEvent(
        &mut self,
        currentEvent: *mut LightColorBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorChangeBeatmapEvent", (currentEvent))?;
        Ok(__cordl_ret)
    }
    pub fn GetColor(
        &mut self,
        colorType: EnvironmentColorType,
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
    pub fn HandleBeatmapCallbacksControllerDidProcessAllCallbacksThisFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCallbacksControllerDidProcessAllCallbacksThisFrame",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleColorBoostBeatmapEvent(
        &mut self,
        colorBoost: *mut ColorBoostBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorBoostBeatmapEvent", (colorBoost))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData,
        colorManager: *mut ColorManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData, colorManager, beatmapCallbacksController))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for BufferedLightColorGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedLightColorGroupEffect_InitData {
    __cordl_parent: crate::System::Object,
    pub lightGroup: *mut LightGroup,
    pub materialPropertyBlockController: *mut MaterialPropertyBlockController,
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BufferedLightColorGroupEffect_InitData => ""
    ."BufferedLightColorGroupEffect/InitData"
);
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
impl crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData {
    pub fn _ctor(
        &mut self,
        lightGroup: *mut LightGroup,
        materialPropertyBlockController: *mut MaterialPropertyBlockController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup, materialPropertyBlockController))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightGroup: *mut LightGroup,
        materialPropertyBlockController: *mut MaterialPropertyBlockController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroup, materialPropertyBlockController))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

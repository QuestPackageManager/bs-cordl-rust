#[cfg(feature = "BufferedLightColorGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedLightColorGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub _materialPropertyBlockController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockController,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _colorBoostBeatmapDataCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _lightColorBeatmapEventCallbackWrappers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
        >,
    >,
    pub _lastIndex: i32,
    pub _timesBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _colorsBuffer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub _elementIdsBuffer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _useBoostColors: bool,
    pub _didReceiveEventThisFrame: bool,
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BufferedLightColorGroupEffect
    => ""."BufferedLightColorGroupEffect"
);
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::BufferedLightColorGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::BufferedLightColorGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl crate::GlobalNamespace::BufferedLightColorGroupEffect {
    pub const kBufferSize: i32 = 24i32;
    #[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cleanup", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorBoostBeatmapEvent(
        &mut self,
        colorBoost: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorBoostBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorBoostBeatmapEvent", (colorBoost))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorChangeBeatmapEvent(
        &mut self,
        currentEvent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightColorBeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorChangeBeatmapEvent", (currentEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData,
        >,
        colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initData, colorManager, beatmapCallbacksController))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData,
        >,
        colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, colorManager, beatmapCallbacksController))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BufferedLightColorGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BufferedLightColorGroupEffect {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroup>,
    pub materialPropertyBlockController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialPropertyBlockController,
    >,
}
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BufferedLightColorGroupEffect_InitData => ""
    ."BufferedLightColorGroupEffect/InitData"
);
#[cfg(feature = "BufferedLightColorGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::BufferedLightColorGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroup>,
        materialPropertyBlockController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MaterialPropertyBlockController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightGroup, materialPropertyBlockController))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightGroup: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightGroup>,
        materialPropertyBlockController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MaterialPropertyBlockController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightGroup, materialPropertyBlockController))?;
        Ok(__cordl_ret.into())
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

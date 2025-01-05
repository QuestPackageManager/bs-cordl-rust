#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct WhiteColorOrAlphaGroupEffect {
    __cordl_parent: crate::GlobalNamespace::LightColorGroupEffect,
    pub _defaultColor: crate::UnityEngine::Color,
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::WhiteColorOrAlphaGroupEffect =>
    ""."WhiteColorOrAlphaGroupEffect"
);
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    type Target = crate::GlobalNamespace::LightColorGroupEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
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
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightColorGroupEffect_InitData,
        >,
        defaultColor: crate::UnityEngine::Color,
        lightManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightWithIdManager,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        bpmController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBpmController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    initData,
                    defaultColor,
                    lightManager,
                    tweeningManager,
                    colorManager,
                    beatmapCallbacksController,
                    bpmController,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightColorGroupEffect_InitData,
        >,
        defaultColor: crate::UnityEngine::Color,
        lightManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightWithIdManager,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
        bpmController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBpmController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    initData,
                    defaultColor,
                    lightManager,
                    tweeningManager,
                    colorManager,
                    beatmapCallbacksController,
                    bpmController,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

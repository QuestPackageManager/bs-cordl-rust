#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct WhiteColorOrAlphaGroupEffect {
    __cordl_parent: crate::GlobalNamespace::LightColorGroupEffect,
    pub _defaultColor: crate::UnityEngine::Color,
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "WhiteColorOrAlphaGroupEffect";
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
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    type Target = crate::GlobalNamespace::LightColorGroupEffect;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WhiteColorOrAlphaGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::WhiteColorOrAlphaGroupEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::GlobalNamespace::EnvironmentColorType, bool, f32),
                        crate::UnityEngine::Color,
                        3usize,
                    >("GetColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetColor", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            method.invoke_unchecked(self, (colorType, colorBoost, brightness))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LightColorGroupEffect_InitData,
                            >,
                            crate::UnityEngine::Color,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LightWithIdManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Tweening::SongTimeTweeningManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ColorManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCallbacksController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBpmController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        initData,
                        defaultColor,
                        lightManager,
                        tweeningManager,
                        colorManager,
                        beatmapCallbacksController,
                        bpmController,
                    ),
                )?
        };
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

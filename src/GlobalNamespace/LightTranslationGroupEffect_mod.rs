#[cfg(feature = "LightTranslationGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroupEffect {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub _beatmapCallbacksController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCallbacksController,
    >,
    pub _transformMask: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                crate::UnityEngine::Vector3,
            >,
        >,
    >,
    pub _xTranslationTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _yTranslationTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _zTranslationTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
    pub _lightTranslationXBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _lightTranslationYBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
    pub _lightTranslationZBeatmapEventCallbackWrapper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    >,
}
#[cfg(feature = "LightTranslationGroupEffect")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightTranslationGroupEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightTranslationGroupEffect";
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
#[cfg(feature = "LightTranslationGroupEffect")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationGroupEffect {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightTranslationGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl crate::GlobalNamespace::LightTranslationGroupEffect {
    #[cfg(feature = "LightTranslationGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::LightTranslationGroupEffect_InitData;
    pub fn Cleanup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Cleanup", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTranslation(
        translation: f32,
        translationLimits: crate::UnityEngine::Vector2,
        distribution: f32,
        distributionLimits: crate::UnityEngine::Vector2,
        mirrored: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            f32,
                            crate::UnityEngine::Vector2,
                            f32,
                            crate::UnityEngine::Vector2,
                            bool,
                        ),
                        f32,
                        5usize,
                    >("ComputeTranslation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ComputeTranslation", 5usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        translation,
                        translationLimits,
                        distribution,
                        distributionLimits,
                        mirrored,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTranslationEventHandler(
        &mut self,
        translationTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
        translationLimits: crate::UnityEngine::Vector2,
        distributionLimits: crate::UnityEngine::Vector2,
        mirrored: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallback_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightTranslationBeatmapEventData,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataCallback_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LightTranslationBeatmapEventData,
                                >,
                            >,
                        >,
                        4usize,
                    >("GetTranslationEventHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTranslationEventHandler", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataCallback_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightTranslationBeatmapEventData,
                >,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (translationTween, translationLimits, distributionLimits, mirrored),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightTranslationGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initData, tweeningManager, beatmapCallbacksController),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetTranslation(
        &mut self,
        _cordl__: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetTranslation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTranslation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl__))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTweenData(
        tween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
        from: f32,
        to: f32,
        startTime: f32,
        endTime: f32,
        easeType: crate::GlobalNamespace::EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
                            f32,
                            f32,
                            f32,
                            f32,
                            crate::GlobalNamespace::EaseType,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetTweenData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetTweenData", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tween, from, to, startTime, endTime, easeType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightTranslationGroupEffect_InitData,
        >,
        tweeningManager: quest_hook::libil2cpp::Gc<
            crate::Tweening::SongTimeTweeningManager,
        >,
        beatmapCallbacksController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCallbacksController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LightTranslationGroupEffect_InitData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Tweening::SongTimeTweeningManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCallbacksController,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (initData, tweeningManager, beatmapCallbacksController),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTranslationGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroupEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub groupId: i32,
    pub elementId: i32,
    pub xMirrored: bool,
    pub yMirrored: bool,
    pub zMirrored: bool,
    pub xTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub yTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub zTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub xTranslationLimits: crate::UnityEngine::Vector2,
    pub xDistributionLimits: crate::UnityEngine::Vector2,
    pub yTranslationLimits: crate::UnityEngine::Vector2,
    pub yDistributionLimits: crate::UnityEngine::Vector2,
    pub zTranslationLimits: crate::UnityEngine::Vector2,
    pub zDistributionLimits: crate::UnityEngine::Vector2,
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightTranslationGroupEffect/InitData";
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
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
impl crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    pub fn New(
        groupId: i32,
        elementId: i32,
        xMirrored: bool,
        yMirrored: bool,
        zMirrored: bool,
        xTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        yTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        zTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        xTranslationLimits: crate::UnityEngine::Vector2,
        xDistributionLimits: crate::UnityEngine::Vector2,
        yTranslationLimits: crate::UnityEngine::Vector2,
        yDistributionLimits: crate::UnityEngine::Vector2,
        zTranslationLimits: crate::UnityEngine::Vector2,
        zDistributionLimits: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    groupId,
                    elementId,
                    xMirrored,
                    yMirrored,
                    zMirrored,
                    xTransform,
                    yTransform,
                    zTransform,
                    xTranslationLimits,
                    xDistributionLimits,
                    yTranslationLimits,
                    yDistributionLimits,
                    zTranslationLimits,
                    zDistributionLimits,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        xMirrored: bool,
        yMirrored: bool,
        zMirrored: bool,
        xTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        yTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        zTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        xTranslationLimits: crate::UnityEngine::Vector2,
        xDistributionLimits: crate::UnityEngine::Vector2,
        yTranslationLimits: crate::UnityEngine::Vector2,
        yDistributionLimits: crate::UnityEngine::Vector2,
        zTranslationLimits: crate::UnityEngine::Vector2,
        zDistributionLimits: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            bool,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                        ),
                        quest_hook::libil2cpp::Void,
                        14usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 14usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        groupId,
                        elementId,
                        xMirrored,
                        yMirrored,
                        zMirrored,
                        xTransform,
                        yTransform,
                        zTransform,
                        xTranslationLimits,
                        xDistributionLimits,
                        yTranslationLimits,
                        yDistributionLimits,
                        zTranslationLimits,
                        zDistributionLimits,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

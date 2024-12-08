#[cfg(feature = "LightTranslationGroupEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroupEffect_InitData {
    __cordl_parent: crate::System::Object,
    pub groupId: i32,
    pub elementId: i32,
    pub xMirrored: bool,
    pub yMirrored: bool,
    pub zMirrored: bool,
    pub xTransform: *mut crate::UnityEngine::Transform,
    pub yTransform: *mut crate::UnityEngine::Transform,
    pub zTransform: *mut crate::UnityEngine::Transform,
    pub xTranslationLimits: crate::UnityEngine::Vector2,
    pub xDistributionLimits: crate::UnityEngine::Vector2,
    pub yTranslationLimits: crate::UnityEngine::Vector2,
    pub yDistributionLimits: crate::UnityEngine::Vector2,
    pub zTranslationLimits: crate::UnityEngine::Vector2,
    pub zDistributionLimits: crate::UnityEngine::Vector2,
}
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightTranslationGroupEffect_InitData => ""
    ."LightTranslationGroupEffect/InitData"
);
#[cfg(feature = "LightTranslationGroupEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationGroupEffect_InitData {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        groupId: i32,
        elementId: i32,
        xMirrored: bool,
        yMirrored: bool,
        zMirrored: bool,
        xTransform: *mut crate::UnityEngine::Transform,
        yTransform: *mut crate::UnityEngine::Transform,
        zTransform: *mut crate::UnityEngine::Transform,
        xTranslationLimits: crate::UnityEngine::Vector2,
        xDistributionLimits: crate::UnityEngine::Vector2,
        yTranslationLimits: crate::UnityEngine::Vector2,
        yDistributionLimits: crate::UnityEngine::Vector2,
        zTranslationLimits: crate::UnityEngine::Vector2,
        zDistributionLimits: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret)
    }
    pub fn New(
        groupId: i32,
        elementId: i32,
        xMirrored: bool,
        yMirrored: bool,
        zMirrored: bool,
        xTransform: *mut crate::UnityEngine::Transform,
        yTransform: *mut crate::UnityEngine::Transform,
        zTransform: *mut crate::UnityEngine::Transform,
        xTranslationLimits: crate::UnityEngine::Vector2,
        xDistributionLimits: crate::UnityEngine::Vector2,
        yTranslationLimits: crate::UnityEngine::Vector2,
        yDistributionLimits: crate::UnityEngine::Vector2,
        zTranslationLimits: crate::UnityEngine::Vector2,
        zDistributionLimits: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
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
#[cfg(feature = "LightTranslationGroupEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroupEffect {
    __cordl_parent: crate::System::Object,
    pub _tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _transformMask: *mut crate::System::Collections::Generic::List_1<
        crate::System::ValueTuple_2<
            *mut crate::UnityEngine::Transform,
            crate::UnityEngine::Vector3,
        >,
    >,
    pub _xTranslationTween: *mut crate::Tweening::FloatTween,
    pub _yTranslationTween: *mut crate::Tweening::FloatTween,
    pub _zTranslationTween: *mut crate::Tweening::FloatTween,
    pub _lightTranslationXBeatmapEventCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _lightTranslationYBeatmapEventCallbackWrapper: *mut BeatmapDataCallbackWrapper,
    pub _lightTranslationZBeatmapEventCallbackWrapper: *mut BeatmapDataCallbackWrapper,
}
#[cfg(feature = "LightTranslationGroupEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightTranslationGroupEffect => ""
    ."LightTranslationGroupEffect"
);
#[cfg(feature = "LightTranslationGroupEffect")]
impl std::ops::Deref for LightTranslationGroupEffect {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl std::ops::DerefMut for LightTranslationGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl LightTranslationGroupEffect {
    #[cfg(feature = "LightTranslationGroupEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::LightTranslationGroupEffect_InitData;
    #[cfg(feature = "LightTranslationGroupEffect+__c__DisplayClass12_0")]
    pub type __c__DisplayClass12_0 = crate::GlobalNamespace::LightTranslationGroupEffect___c__DisplayClass12_0;
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
    pub fn _ctor(
        &mut self,
        initData: *mut crate::GlobalNamespace::LightTranslationGroupEffect_InitData,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initData, tweeningManager, beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
    pub fn GetTranslationEventHandler(
        &mut self,
        translationTween: *mut crate::Tweening::FloatTween,
        translationLimits: crate::UnityEngine::Vector2,
        distributionLimits: crate::UnityEngine::Vector2,
        mirrored: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut BeatmapDataCallback_1<*mut LightTranslationBeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataCallback_1<
            *mut LightTranslationBeatmapEventData,
        > = __cordl_object
            .invoke(
                "GetTranslationEventHandler",
                (translationTween, translationLimits, distributionLimits, mirrored),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetTranslation(
        &mut self,
        _: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTranslation", (_))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        initData: *mut crate::GlobalNamespace::LightTranslationGroupEffect_InitData,
        tweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
        beatmapCallbacksController: *mut BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (initData, tweeningManager, beatmapCallbacksController),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightTranslationGroupEffect")]
impl quest_hook::libil2cpp::ObjectType for LightTranslationGroupEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

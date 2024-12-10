#[cfg(feature = "BeatmapObjectsAvoidance")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectsAvoidance {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _zOffset: f32,
    pub _yOffset: f32,
    pub _gravity: crate::UnityEngine::Vector2,
    pub _towardsPlayerWrapperTransform: *mut crate::UnityEngine::Transform,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _beatmapData: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
    pub _beatmapObjectSpawnController: *mut crate::GlobalNamespace::IBeatmapObjectSpawnController,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
    pub _avoidanceYOffsetEvaluatorProvider: *mut crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator,
    pub _pathEvaluator: *mut crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator,
    pub _tiltEvaluator: *mut crate::GlobalNamespace::BeatmapObjectAvoidanceTiltEvaluator,
    pub _pathBezierSplineEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
    pub _accelerationBezierSplineEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
    pub _transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapObjectsAvoidance => ""
    ."BeatmapObjectsAvoidance"
);
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl crate::GlobalNamespace::BeatmapObjectsAvoidance {
    pub fn AdjustPositionWithOffsetDirection(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        lineIndex: i32,
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AdjustPositionWithOffsetDirection",
                (position, lineIndex, offsetDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildAnimationCurvePath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BuildAnimationCurvePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapObjectSpawnControllerDidInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatmapObjectSpawnControllerDidInit", ())?;
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
    pub fn SetupAndRun(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupAndRun", ())?;
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
#[cfg(feature = "BeatmapObjectsAvoidance")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectsAvoidance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

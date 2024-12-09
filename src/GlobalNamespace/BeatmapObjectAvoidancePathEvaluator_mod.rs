#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectAvoidancePathEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _jumpStartZ: f32,
    pub _jumpEndZ: f32,
    pub _zOffset: f32,
    pub _yOffset: f32,
    pub _noteJumpSpeed: f32,
    pub _moveToPlayerHeadTParam: f32,
    pub _pathBezierCurveEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator => ""
    ."BeatmapObjectAvoidancePathEvaluator"
);
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    pub fn GetCurrentPathPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("GetCurrentPathPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
        playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
        pathBezierCurveEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
        jumpStartZ: f32,
        jumpEndZ: f32,
        yOffset: f32,
        zOffset: f32,
        noteJumpSeed: f32,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    audioTimeSource,
                    playerTransforms,
                    pathBezierCurveEvaluator,
                    jumpStartZ,
                    jumpEndZ,
                    yOffset,
                    zOffset,
                    noteJumpSeed,
                    moveToPlayerHeadTParam,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
        playerTransforms: *mut crate::GlobalNamespace::PlayerTransforms,
        pathBezierCurveEvaluator: *mut crate::GlobalNamespace::BezierSplineEvaluator,
        jumpStartZ: f32,
        jumpEndZ: f32,
        yOffset: f32,
        zOffset: f32,
        noteJumpSeed: f32,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    audioTimeSource,
                    playerTransforms,
                    pathBezierCurveEvaluator,
                    jumpStartZ,
                    jumpEndZ,
                    yOffset,
                    zOffset,
                    noteJumpSeed,
                    moveToPlayerHeadTParam,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

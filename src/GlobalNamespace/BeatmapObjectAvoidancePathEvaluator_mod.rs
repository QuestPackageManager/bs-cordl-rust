#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectAvoidancePathEvaluator {
    __cordl_parent: crate::System::Object,
    pub _jumpStartZ: f32,
    pub _jumpEndZ: f32,
    pub _zOffset: f32,
    pub _yOffset: f32,
    pub _noteJumpSpeed: f32,
    pub _moveToPlayerHeadTParam: f32,
    pub _pathBezierCurveEvaluator: *mut BezierSplineEvaluator,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _playerTransforms: *mut PlayerTransforms,
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapObjectAvoidancePathEvaluator => ""
    ."BeatmapObjectAvoidancePathEvaluator"
);
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::Deref for BeatmapObjectAvoidancePathEvaluator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::DerefMut for BeatmapObjectAvoidancePathEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl BeatmapObjectAvoidancePathEvaluator {
    pub fn _ctor(
        &mut self,
        audioTimeSource: *mut IAudioTimeSource,
        playerTransforms: *mut PlayerTransforms,
        pathBezierCurveEvaluator: *mut BezierSplineEvaluator,
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
        audioTimeSource: *mut IAudioTimeSource,
        playerTransforms: *mut PlayerTransforms,
        pathBezierCurveEvaluator: *mut BezierSplineEvaluator,
        jumpStartZ: f32,
        jumpEndZ: f32,
        yOffset: f32,
        zOffset: f32,
        noteJumpSeed: f32,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl quest_hook::libil2cpp::ObjectType for BeatmapObjectAvoidancePathEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

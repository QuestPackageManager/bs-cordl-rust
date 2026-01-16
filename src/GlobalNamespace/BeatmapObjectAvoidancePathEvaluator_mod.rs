#[cfg(feature = "cordl_class_BeatmapObjectAvoidancePathEvaluator")]
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
    pub _pathBezierCurveEvaluator:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierSplineEvaluator>,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAudioTimeSource>,
    pub _playerTransforms: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerTransforms>,
}
#[cfg(feature = "cordl_class_BeatmapObjectAvoidancePathEvaluator")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectAvoidancePathEvaluator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidancePathEvaluator")]
impl crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator {
    pub fn GetCurrentPathPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Vector3, 0usize>(
                        "GetCurrentPathPosition",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCurrentPathPosition",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector3 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        audioTimeSource: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAudioTimeSource>,
        playerTransforms: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerTransforms>,
        pathBezierCurveEvaluator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BezierSplineEvaluator,
        >,
        jumpStartZ: f32,
        jumpEndZ: f32,
        yOffset: f32,
        zOffset: f32,
        noteJumpSeed: f32,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioTimeSource: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAudioTimeSource>,
        playerTransforms: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerTransforms>,
        pathBezierCurveEvaluator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BezierSplineEvaluator,
        >,
        jumpStartZ: f32,
        jumpEndZ: f32,
        yOffset: f32,
        zOffset: f32,
        noteJumpSeed: f32,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IAudioTimeSource>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerTransforms>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BezierSplineEvaluator>,
                        f32,
                        f32,
                        f32,
                        f32,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 9usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
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
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapObjectAvoidancePathEvaluator")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::BeatmapObjectAvoidancePathEvaluator
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

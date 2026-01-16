#[cfg(feature = "cordl_class_UnityEngine+Rendering+KeyframeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyframeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+KeyframeUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::KeyframeUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "KeyframeUtility";
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
#[cfg(feature = "UnityEngine+Rendering+KeyframeUtility")]
impl std::ops::Deref for crate::UnityEngine::Rendering::KeyframeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+KeyframeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::KeyframeUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+KeyframeUtility")]
impl crate::UnityEngine::Rendering::KeyframeUtility {
    pub fn EvalCurveSegmentAndDeriv(
        dstValue: quest_hook::libil2cpp::ByRefMut<f32>,
        dstDeriv: quest_hook::libil2cpp::ByRefMut<f32>,
        lhsKey: crate::UnityEngine::Keyframe,
        rhsKey: crate::UnityEngine::Keyframe,
        desiredTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            crate::UnityEngine::Keyframe,
                            crate::UnityEngine::Keyframe,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("EvalCurveSegmentAndDeriv")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EvalCurveSegmentAndDeriv", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (dstValue, dstDeriv, lhsKey, rhsKey, desiredTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EvalKeyAtTime(
        keys: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Keyframe>,
        lhsIndex: i32,
        rhsIndex: i32,
        startTime: f32,
        endTime: f32,
        currTime: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Keyframe> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Keyframe,
                            >,
                            i32,
                            i32,
                            f32,
                            f32,
                            f32,
                        ),
                        crate::UnityEngine::Keyframe,
                        6usize,
                    >("EvalKeyAtTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EvalKeyAtTime", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Keyframe = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (keys, lhsIndex, rhsIndex, startTime, endTime, currTime),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FetchKeyFromIndexClampEdge(
        keys: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Keyframe>,
        index: i32,
        segmentStartTime: f32,
        segmentEndTime: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Keyframe> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Keyframe,
                            >,
                            i32,
                            f32,
                            f32,
                        ),
                        crate::UnityEngine::Keyframe,
                        4usize,
                    >("FetchKeyFromIndexClampEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FetchKeyFromIndexClampEdge", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Keyframe = unsafe {
            cordl_method_info
                .invoke_unchecked((), (keys, index, segmentStartTime, segmentEndTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyframeAndClampEdge(
        keys: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Keyframe>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Keyframe> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Keyframe,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::Keyframe,
                        2usize,
                    >("GetKeyframeAndClampEdge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetKeyframeAndClampEdge", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Keyframe = unsafe {
            cordl_method_info.invoke_unchecked((), (keys, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InterpAnimationCurve(
        lhsAndResultCurve: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
        >,
        rhsCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::AnimationCurve,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::AnimationCurve,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InterpAnimationCurve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InterpAnimationCurve", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (lhsAndResultCurve, rhsCurve, t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LerpSingleKeyframe(
        lhs: crate::UnityEngine::Keyframe,
        rhs: crate::UnityEngine::Keyframe,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Keyframe> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Keyframe,
                            crate::UnityEngine::Keyframe,
                            f32,
                        ),
                        crate::UnityEngine::Keyframe,
                        3usize,
                    >("LerpSingleKeyframe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LerpSingleKeyframe", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Keyframe = unsafe {
            cordl_method_info.invoke_unchecked((), (lhs, rhs, t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResetAnimationCurve(
        curve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetAnimationCurve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetAnimationCurve", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (curve))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+KeyframeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::KeyframeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

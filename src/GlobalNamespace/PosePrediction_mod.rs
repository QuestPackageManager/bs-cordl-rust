#[cfg(feature = "PosePrediction")]
#[repr(C)]
#[derive(Debug)]
pub struct PosePrediction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PosePrediction")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PosePrediction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PosePrediction";
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
#[cfg(feature = "PosePrediction")]
impl std::ops::Deref for crate::GlobalNamespace::PosePrediction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PosePrediction")]
impl std::ops::DerefMut for crate::GlobalNamespace::PosePrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PosePrediction")]
impl crate::GlobalNamespace::PosePrediction {
    pub fn InterpolatePose(
        prev: crate::UnityEngine::Pose,
        curr: crate::UnityEngine::Pose,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::UnityEngine::Pose, crate::UnityEngine::Pose, f32),
                        crate::UnityEngine::Pose,
                        3usize,
                    >("InterpolatePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InterpolatePose", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method.invoke_unchecked((), (prev, curr, t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InterpolatePoseSerializable(
        a: crate::GlobalNamespace::PoseSerializable,
        b: crate::GlobalNamespace::PoseSerializable,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::PoseSerializable,
                            crate::GlobalNamespace::PoseSerializable,
                            f32,
                        ),
                        crate::GlobalNamespace::PoseSerializable,
                        3usize,
                    >("InterpolatePoseSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InterpolatePoseSerializable", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = unsafe {
            method.invoke_unchecked((), (a, b, t))?
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
    pub fn PredictPose(
        prev: crate::UnityEngine::Pose,
        prevTime: i64,
        curr: crate::UnityEngine::Pose,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Pose,
                            i64,
                            crate::UnityEngine::Pose,
                            i64,
                            i64,
                        ),
                        crate::UnityEngine::Pose,
                        5usize,
                    >("PredictPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PredictPose", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pose = unsafe {
            method.invoke_unchecked((), (prev, prevTime, curr, currTime, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PredictPoseSerializable(
        prev: crate::GlobalNamespace::PoseSerializable,
        prevTime: i64,
        curr: crate::GlobalNamespace::PoseSerializable,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PoseSerializable> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::PoseSerializable,
                            i64,
                            crate::GlobalNamespace::PoseSerializable,
                            i64,
                            i64,
                        ),
                        crate::GlobalNamespace::PoseSerializable,
                        5usize,
                    >("PredictPoseSerializable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PredictPoseSerializable", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::PoseSerializable = unsafe {
            method.invoke_unchecked((), (prev, prevTime, curr, currTime, _cordl_time))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PosePrediction")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PosePrediction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

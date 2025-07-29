#[cfg(feature = "cordl_class_UnityEngine+SpatialTracking+PoseDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+SpatialTracking+PoseDataSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SpatialTracking::PoseDataSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SpatialTracking";
    const CLASS_NAME: &'static str = "PoseDataSource";
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
#[cfg(feature = "cordl_class_UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::Deref for crate::UnityEngine::SpatialTracking::PoseDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::DerefMut for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl crate::UnityEngine::SpatialTracking::PoseDataSource {
    pub fn GetDataFromSource(
        poseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
                        ),
                        crate::UnityEngine::SpatialTracking::PoseDataFlags,
                        2usize,
                    >("GetDataFromSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDataFromSource", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = unsafe {
            cordl_method_info.invoke_unchecked((), (poseSource, resultPose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNodePoseData(
        node: crate::UnityEngine::XR::XRNode,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::SpatialTracking::PoseDataFlags,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::XR::XRNode,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
                        ),
                        crate::UnityEngine::SpatialTracking::PoseDataFlags,
                        2usize,
                    >("GetNodePoseData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetNodePoseData", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::SpatialTracking::PoseDataFlags = unsafe {
            cordl_method_info.invoke_unchecked((), (node, resultPose))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDataFromSource(
        poseSource: crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        resultPose: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Pose>,
                        ),
                        bool,
                        2usize,
                    >("TryGetDataFromSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetDataFromSource", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (poseSource, resultPose))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+SpatialTracking+PoseDataSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

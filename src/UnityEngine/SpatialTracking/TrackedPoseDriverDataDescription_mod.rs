#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedPoseDriverDataDescription {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriverDataDescription"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl std::ops::Deref
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl std::ops::DerefMut
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    #[cfg(
        feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData"
    )]
    pub type PoseData = crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TrackedPoseDriverDataDescription_PoseData {
    pub PoseNames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub Poses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::SpatialTracking::TrackedPoseDriver_TrackedPose,
        >,
    >,
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData =>
    "UnityEngine.SpatialTracking"."TrackedPoseDriverDataDescription/PoseData"
);
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+TrackedPoseDriverDataDescription+PoseData")]
impl crate::UnityEngine::SpatialTracking::TrackedPoseDriverDataDescription_PoseData {}

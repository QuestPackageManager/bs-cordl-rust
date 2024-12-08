#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseDataSource {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SpatialTracking::PoseDataSource =>
    "UnityEngine.SpatialTracking"."PoseDataSource"
);
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::Deref for crate::UnityEngine::SpatialTracking::PoseDataSource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl std::ops::DerefMut for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl crate::UnityEngine::SpatialTracking::PoseDataSource {}
#[cfg(feature = "UnityEngine+SpatialTracking+PoseDataSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SpatialTracking::PoseDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
#[repr(C)]
#[derive(Debug)]
pub struct DownloadOnlyLocation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceLocations::LocationWrapper,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::DownloadOnlyLocation =>
    "UnityEngine.ResourceManagement.ResourceProviders"."DownloadOnlyLocation"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::DownloadOnlyLocation {
    type Target = crate::UnityEngine::ResourceManagement::ResourceLocations::LocationWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::DownloadOnlyLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::DownloadOnlyLocation {
    pub fn _ctor(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (location))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (location))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+DownloadOnlyLocation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::DownloadOnlyLocation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

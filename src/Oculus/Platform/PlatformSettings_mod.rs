#[cfg(feature = "Oculus+Platform+PlatformSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub ovrAppID: *mut crate::System::String,
    pub ovrMobileAppID: *mut crate::System::String,
    pub ovrUseStandalonePlatform: bool,
}
#[cfg(feature = "Oculus+Platform+PlatformSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::PlatformSettings =>
    "Oculus.Platform"."PlatformSettings"
);
#[cfg(feature = "Oculus+Platform+PlatformSettings")]
impl std::ops::Deref for crate::Oculus::Platform::PlatformSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformSettings")]
impl std::ops::DerefMut for crate::Oculus::Platform::PlatformSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+PlatformSettings")]
impl crate::Oculus::Platform::PlatformSettings {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+PlatformSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::PlatformSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

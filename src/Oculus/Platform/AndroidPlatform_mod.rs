#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidPlatform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AndroidPlatform =>
    "Oculus.Platform"."AndroidPlatform"
);
#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
impl std::ops::Deref for crate::Oculus::Platform::AndroidPlatform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
impl std::ops::DerefMut for crate::Oculus::Platform::AndroidPlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
impl crate::Oculus::Platform::AndroidPlatform {
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
    pub fn Initialize(
        &mut self,
        appId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Initialize", (appId))?;
        Ok(__cordl_ret)
    }
    pub fn AsyncInitialize(
        &mut self,
        appId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Request_1<
            *mut crate::Oculus::Platform::Models::PlatformInitialize,
        > = __cordl_object.invoke("AsyncInitialize", (appId))?;
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
#[cfg(feature = "Oculus+Platform+AndroidPlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::AndroidPlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

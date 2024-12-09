#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVoipState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub MicrophoneMuted: crate::Oculus::Platform::VoipMuteState,
    pub Status: crate::Oculus::Platform::SystemVoipStatus,
}
#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::SystemVoipState =>
    "Oculus.Platform.Models"."SystemVoipState"
);
#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
impl std::ops::Deref for crate::Oculus::Platform::Models::SystemVoipState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::SystemVoipState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
impl crate::Oculus::Platform::Models::SystemVoipState {
    pub fn New(o: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (o))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+Models+SystemVoipState")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::SystemVoipState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

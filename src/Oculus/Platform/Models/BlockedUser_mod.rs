#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
#[repr(C)]
#[derive(Debug)]
pub struct BlockedUser {
    __cordl_parent: crate::System::Object,
    pub Id: u64,
}
#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::BlockedUser =>
    "Oculus.Platform.Models"."BlockedUser"
);
#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
impl std::ops::Deref for crate::Oculus::Platform::Models::BlockedUser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::BlockedUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
impl crate::Oculus::Platform::Models::BlockedUser {
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
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Oculus+Platform+Models+BlockedUser")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::BlockedUser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

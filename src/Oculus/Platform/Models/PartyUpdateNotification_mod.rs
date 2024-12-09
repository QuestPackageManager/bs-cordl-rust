#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyUpdateNotification {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Action: crate::Oculus::Platform::PartyUpdateAction,
    pub PartyId: u64,
    pub SenderId: u64,
    pub UpdateTimestamp: *mut quest_hook::libil2cpp::Il2CppString,
    pub UserAlias: *mut quest_hook::libil2cpp::Il2CppString,
    pub UserId: u64,
    pub UserName: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Models::PartyUpdateNotification => "Oculus.Platform.Models"
    ."PartyUpdateNotification"
);
#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
impl std::ops::Deref for crate::Oculus::Platform::Models::PartyUpdateNotification {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::PartyUpdateNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
impl crate::Oculus::Platform::Models::PartyUpdateNotification {
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
#[cfg(feature = "Oculus+Platform+Models+PartyUpdateNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::PartyUpdateNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

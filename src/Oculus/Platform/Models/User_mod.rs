#[cfg(feature = "Oculus+Platform+Models+User")]
#[repr(C)]
#[derive(Debug)]
pub struct User {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub DisplayName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cordl_ID: u64,
    pub ImageURL: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub OculusID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Presence: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PresenceDeeplinkMessage: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub PresenceDestinationApiName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub PresenceLobbySessionId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub PresenceMatchSessionId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub PresenceStatus: crate::Oculus::Platform::UserPresenceStatus,
    pub SmallImageUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "Oculus+Platform+Models+User")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::User =>
    "Oculus.Platform.Models"."User"
);
#[cfg(feature = "Oculus+Platform+Models+User")]
impl std::ops::Deref for crate::Oculus::Platform::Models::User {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+User")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::User {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+User")]
impl crate::Oculus::Platform::Models::User {
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+User")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::User {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

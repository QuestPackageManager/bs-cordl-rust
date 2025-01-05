#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
#[repr(C)]
#[derive(Debug)]
pub struct ApplicationInvite {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub DestinationOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::Destination,
    >,
    pub Destination: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::Destination,
    >,
    pub _cordl_ID: u64,
    pub IsActive: bool,
    pub LobbySessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub MatchSessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub RecipientOptional: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::User,
    >,
    pub Recipient: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
}
#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::ApplicationInvite =>
    "Oculus.Platform.Models"."ApplicationInvite"
);
#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
impl std::ops::Deref for crate::Oculus::Platform::Models::ApplicationInvite {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::ApplicationInvite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
impl crate::Oculus::Platform::Models::ApplicationInvite {
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
#[cfg(feature = "Oculus+Platform+Models+ApplicationInvite")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::ApplicationInvite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

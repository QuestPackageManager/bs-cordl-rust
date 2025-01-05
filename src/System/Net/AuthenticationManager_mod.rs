#[cfg(feature = "System+Net+AuthenticationManager")]
#[repr(C)]
#[derive(Debug)]
pub struct AuthenticationManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Net+AuthenticationManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::AuthenticationManager =>
    "System.Net"."AuthenticationManager"
);
#[cfg(feature = "System+Net+AuthenticationManager")]
impl std::ops::Deref for crate::System::Net::AuthenticationManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl std::ops::DerefMut for crate::System::Net::AuthenticationManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl crate::System::Net::AuthenticationManager {
    pub fn Authenticate(
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Authenticate", (challenge, request, credentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoAuthenticate(
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoAuthenticate", (challenge, request, credentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureModules() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureModules", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PreAuthenticate(
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PreAuthenticate", (request, credentials))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+AuthenticationManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::AuthenticationManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

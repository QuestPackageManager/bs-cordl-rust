#[cfg(feature = "Mono+Http+NtlmSession")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmSession {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub message: *mut crate::Mono::Security::Protocol::Ntlm::MessageBase,
}
#[cfg(feature = "Mono+Http+NtlmSession")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Http::NtlmSession => "Mono.Http"
    ."NtlmSession"
);
#[cfg(feature = "Mono+Http+NtlmSession")]
impl std::ops::Deref for crate::Mono::Http::NtlmSession {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Http+NtlmSession")]
impl std::ops::DerefMut for crate::Mono::Http::NtlmSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Http+NtlmSession")]
impl crate::Mono::Http::NtlmSession {
    pub fn Authenticate(
        &mut self,
        challenge: *mut quest_hook::libil2cpp::Il2CppString,
        webRequest: *mut crate::System::Net::WebRequest,
        credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Authorization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Authorization = __cordl_object
            .invoke("Authenticate", (challenge, webRequest, credentials))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
}
#[cfg(feature = "Mono+Http+NtlmSession")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Http::NtlmSession {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

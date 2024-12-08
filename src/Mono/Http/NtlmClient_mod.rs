#[cfg(feature = "Mono+Http+NtlmClient")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmClient {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Http+NtlmClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Http::NtlmClient => "Mono.Http"
    ."NtlmClient"
);
#[cfg(feature = "Mono+Http+NtlmClient")]
impl std::ops::Deref for crate::Mono::Http::NtlmClient {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Http+NtlmClient")]
impl std::ops::DerefMut for crate::Mono::Http::NtlmClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Http+NtlmClient")]
impl crate::Mono::Http::NtlmClient {
    #[cfg(feature = "Mono+Http+NtlmClient+__c")]
    pub type __c = crate::Mono::Http::NtlmClient___c;
    pub fn Authenticate(
        &mut self,
        challenge: *mut crate::System::String,
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
    pub fn PreAuthenticate(
        &mut self,
        webRequest: *mut crate::System::Net::WebRequest,
        credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Authorization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Authorization = __cordl_object
            .invoke("PreAuthenticate", (webRequest, credentials))?;
        Ok(__cordl_ret)
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
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AuthenticationType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Http+NtlmClient")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Http::NtlmClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
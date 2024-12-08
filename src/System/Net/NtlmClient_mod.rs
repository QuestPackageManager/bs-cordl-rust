#[cfg(feature = "System+Net+NtlmClient")]
#[repr(C)]
#[derive(Debug)]
pub struct NtlmClient {
    __cordl_parent: crate::System::Object,
    pub authObject: *mut crate::System::Net::IAuthenticationModule,
}
#[cfg(feature = "System+Net+NtlmClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NtlmClient => "System.Net"
    ."NtlmClient"
);
#[cfg(feature = "System+Net+NtlmClient")]
impl std::ops::Deref for crate::System::Net::NtlmClient {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl std::ops::DerefMut for crate::System::Net::NtlmClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl crate::System::Net::NtlmClient {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+NtlmClient")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NtlmClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

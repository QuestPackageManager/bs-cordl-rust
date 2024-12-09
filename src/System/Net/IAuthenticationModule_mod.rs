#[cfg(feature = "System+Net+IAuthenticationModule")]
#[repr(C)]
#[derive(Debug)]
pub struct IAuthenticationModule {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+IAuthenticationModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::IAuthenticationModule =>
    "System.Net"."IAuthenticationModule"
);
#[cfg(feature = "System+Net+IAuthenticationModule")]
impl std::ops::Deref for crate::System::Net::IAuthenticationModule {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IAuthenticationModule")]
impl std::ops::DerefMut for crate::System::Net::IAuthenticationModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IAuthenticationModule")]
impl crate::System::Net::IAuthenticationModule {
    pub fn Authenticate(
        &mut self,
        challenge: *mut quest_hook::libil2cpp::Il2CppString,
        request: *mut crate::System::Net::WebRequest,
        credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Authorization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Authorization = __cordl_object
            .invoke("Authenticate", (challenge, request, credentials))?;
        Ok(__cordl_ret)
    }
    pub fn PreAuthenticate(
        &mut self,
        request: *mut crate::System::Net::WebRequest,
        credentials: *mut crate::System::Net::ICredentials,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Authorization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Authorization = __cordl_object
            .invoke("PreAuthenticate", (request, credentials))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_AuthenticationType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+IAuthenticationModule")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::IAuthenticationModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

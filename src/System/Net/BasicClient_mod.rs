#[cfg(feature = "System+Net+BasicClient")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+BasicClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::BasicClient => "System.Net"
    ."BasicClient"
);
#[cfg(feature = "System+Net+BasicClient")]
impl std::ops::Deref for crate::System::Net::BasicClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BasicClient")]
impl std::ops::DerefMut for crate::System::Net::BasicClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+BasicClient")]
impl crate::System::Net::BasicClient {
    pub fn Authenticate(
        &mut self,
        challenge: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        webRequest: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = __cordl_object
            .invoke("Authenticate", (challenge, webRequest, credentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBytes", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalAuthenticate(
        webRequest: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalAuthenticate", (webRequest, credentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PreAuthenticate(
        &mut self,
        webRequest: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Authorization>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Authorization> = __cordl_object
            .invoke("PreAuthenticate", (webRequest, credentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AuthenticationType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+BasicClient")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::BasicClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+BasicClient")]
impl AsRef<crate::System::Net::IAuthenticationModule>
for crate::System::Net::BasicClient {
    fn as_ref(&self) -> &crate::System::Net::IAuthenticationModule {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+BasicClient")]
impl AsMut<crate::System::Net::IAuthenticationModule>
for crate::System::Net::BasicClient {
    fn as_mut(&mut self) -> &mut crate::System::Net::IAuthenticationModule {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "System+Net+NetworkCredential")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkCredential {
    __cordl_parent: crate::System::Object,
    pub m_domain: *mut crate::System::String,
    pub m_userName: *mut crate::System::String,
    pub m_password: *mut crate::System::Security::SecureString,
}
#[cfg(feature = "System+Net+NetworkCredential")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetworkCredential => "System.Net"
    ."NetworkCredential"
);
#[cfg(feature = "System+Net+NetworkCredential")]
impl std::ops::Deref for crate::System::Net::NetworkCredential {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkCredential")]
impl std::ops::DerefMut for crate::System::Net::NetworkCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkCredential")]
impl crate::System::Net::NetworkCredential {
    pub fn GetCredential(
        &mut self,
        uri: *mut crate::System::Uri,
        authType: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::NetworkCredential> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkCredential = __cordl_object
            .invoke("GetCredential", (uri, authType))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetDomain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("InternalGetDomain", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetPassword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("InternalGetPassword", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetUserName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("InternalGetUserName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String1(
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userName, password, domain))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String0(
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userName, password))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_String1(
        &mut self,
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (userName, password, domain))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String0(
        &mut self,
        userName: *mut crate::System::String,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (userName, password))?;
        Ok(__cordl_ret)
    }
    pub fn get_Domain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Domain", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Password(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Password", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_UserName", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Domain(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Domain", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Password(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Password", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_UserName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserName", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+NetworkCredential")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetworkCredential {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

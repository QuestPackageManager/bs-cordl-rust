#[cfg(feature = "System+Net+Mail+MailAddress")]
#[repr(C)]
#[derive(Debug)]
pub struct MailAddress {
    __cordl_parent: crate::System::Object,
    pub _displayNameEncoding: *mut crate::System::Text::Encoding,
    pub _displayName: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _host: *mut crate::System::String,
}
#[cfg(feature = "System+Net+Mail+MailAddress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::MailAddress =>
    "System.Net.Mail"."MailAddress"
);
#[cfg(feature = "System+Net+Mail+MailAddress")]
impl std::ops::Deref for crate::System::Net::Mail::MailAddress {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddress")]
impl std::ops::DerefMut for crate::System::Net::Mail::MailAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddress")]
impl crate::System::Net::Mail::MailAddress {
    pub fn Equals(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String0(
        &mut self,
        displayName: *mut crate::System::String,
        userName: *mut crate::System::String,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (displayName, userName, domain))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        address: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Encoding2(
        &mut self,
        address: *mut crate::System::String,
        displayName: *mut crate::System::String,
        displayNameEncoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address, displayName, displayNameEncoding))?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DisplayName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SmtpAddress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SmtpAddress", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String_String0(
        displayName: *mut crate::System::String,
        userName: *mut crate::System::String,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (displayName, userName, domain))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        address: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Encoding2(
        address: *mut crate::System::String,
        displayName: *mut crate::System::String,
        displayNameEncoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address, displayName, displayNameEncoding))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Mail+MailAddress")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::MailAddress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

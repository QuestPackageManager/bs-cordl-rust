#[cfg(feature = "System+Net+IWebProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct IWebProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+IWebProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::IWebProxy => "System.Net"
    ."IWebProxy"
);
#[cfg(feature = "System+Net+IWebProxy")]
impl std::ops::Deref for crate::System::Net::IWebProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IWebProxy")]
impl std::ops::DerefMut for crate::System::Net::IWebProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IWebProxy")]
impl crate::System::Net::IWebProxy {
    pub fn GetProxy(
        &mut self,
        destination: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("GetProxy", (destination))?;
        Ok(__cordl_ret)
    }
    pub fn IsBypassed(
        &mut self,
        host: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBypassed", (host))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ICredentials> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ICredentials = __cordl_object
            .invoke("get_Credentials", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+IWebProxy")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::IWebProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

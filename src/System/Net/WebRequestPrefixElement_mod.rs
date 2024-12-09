#[cfg(feature = "System+Net+WebRequestPrefixElement")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestPrefixElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Prefix: *mut quest_hook::libil2cpp::Il2CppString,
    pub creator: *mut crate::System::Net::IWebRequestCreate,
    pub creatorType: *mut crate::System::Type,
}
#[cfg(feature = "System+Net+WebRequestPrefixElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebRequestPrefixElement =>
    "System.Net"."WebRequestPrefixElement"
);
#[cfg(feature = "System+Net+WebRequestPrefixElement")]
impl std::ops::Deref for crate::System::Net::WebRequestPrefixElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebRequestPrefixElement")]
impl std::ops::DerefMut for crate::System::Net::WebRequestPrefixElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebRequestPrefixElement")]
impl crate::System::Net::WebRequestPrefixElement {
    pub fn New(
        P: *mut quest_hook::libil2cpp::Il2CppString,
        C: *mut crate::System::Net::IWebRequestCreate,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (P, C))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        P: *mut quest_hook::libil2cpp::Il2CppString,
        C: *mut crate::System::Net::IWebRequestCreate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (P, C))?;
        Ok(__cordl_ret)
    }
    pub fn get_Creator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IWebRequestCreate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IWebRequestCreate = __cordl_object
            .invoke("get_Creator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Creator(
        &mut self,
        value: *mut crate::System::Net::IWebRequestCreate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Creator", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+WebRequestPrefixElement")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebRequestPrefixElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

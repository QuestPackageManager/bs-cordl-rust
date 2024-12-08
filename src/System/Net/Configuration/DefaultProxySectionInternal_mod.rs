#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultProxySectionInternal {
    __cordl_parent: crate::System::Object,
    pub webProxy: *mut crate::System::Net::IWebProxy,
}
#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Configuration::DefaultProxySectionInternal =>
    "System.Net.Configuration"."DefaultProxySectionInternal"
);
#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
impl std::ops::Deref for crate::System::Net::Configuration::DefaultProxySectionInternal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
impl std::ops::DerefMut
for crate::System::Net::Configuration::DefaultProxySectionInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
impl crate::System::Net::Configuration::DefaultProxySectionInternal {
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
    pub fn get_WebProxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IWebProxy> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IWebProxy = __cordl_object
            .invoke("get_WebProxy", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Configuration+DefaultProxySectionInternal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::DefaultProxySectionInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

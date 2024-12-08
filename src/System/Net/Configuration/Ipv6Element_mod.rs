#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
#[repr(C)]
#[derive(Debug)]
pub struct Ipv6Element {
    __cordl_parent: crate::System::Configuration::ConfigurationElement,
}
#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Configuration::Ipv6Element =>
    "System.Net.Configuration"."Ipv6Element"
);
#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
impl std::ops::Deref for crate::System::Net::Configuration::Ipv6Element {
    type Target = crate::System::Configuration::ConfigurationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
impl std::ops::DerefMut for crate::System::Net::Configuration::Ipv6Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
impl crate::System::Net::Configuration::Ipv6Element {
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
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Configuration::ConfigurationPropertyCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Configuration::ConfigurationPropertyCollection = __cordl_object
            .invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Configuration+Ipv6Element")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Configuration::Ipv6Element {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

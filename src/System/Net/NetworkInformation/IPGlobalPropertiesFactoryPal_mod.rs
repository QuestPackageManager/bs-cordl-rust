#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
#[repr(C)]
#[derive(Debug)]
pub struct IPGlobalPropertiesFactoryPal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal =>
    "System.Net.NetworkInformation"."IPGlobalPropertiesFactoryPal"
);
#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
impl crate::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal {}
#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

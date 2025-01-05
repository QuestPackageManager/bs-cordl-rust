#[cfg(feature = "System+Net+NetworkInformation+IPGlobalPropertiesFactoryPal")]
#[repr(C)]
#[derive(Debug)]
pub struct IPGlobalPropertiesFactoryPal {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
impl crate::System::Net::NetworkInformation::IPGlobalPropertiesFactoryPal {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::IPGlobalProperties,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::IPGlobalProperties,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
}
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

#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct LinuxUnicastIPAddressInformation {
    __cordl_parent: crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    pub address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation =>
    "System.Net.NetworkInformation"."LinuxUnicastIPAddressInformation"
);
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    type Target = crate::System::Net::NetworkInformation::UnicastIPAddressInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    pub fn New(
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress> = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

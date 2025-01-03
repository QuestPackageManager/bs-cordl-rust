#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct UnixIPInterfaceProperties {
    __cordl_parent: crate::System::Net::NetworkInformation::IPInterfaceProperties,
    pub iface: quest_hook::libil2cpp::Gc<
        crate::System::Net::NetworkInformation::UnixNetworkInterface,
    >,
    pub addresses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::System::Net::IPAddress>,
    >,
}
#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::UnixIPInterfaceProperties =>
    "System.Net.NetworkInformation"."UnixIPInterfaceProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::UnixIPInterfaceProperties {
    type Target = crate::System::Net::NetworkInformation::IPInterfaceProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::UnixIPInterfaceProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
impl crate::System::Net::NetworkInformation::UnixIPInterfaceProperties {
    pub fn New(
        iface: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnixNetworkInterface,
        >,
        addresses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Net::IPAddress,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (iface, addresses))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        iface: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnixNetworkInterface,
        >,
        addresses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Net::IPAddress,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (iface, addresses))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnicastAddresses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::UnicastIPAddressInformationCollection,
        > = __cordl_object.invoke("get_UnicastAddresses", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixIPInterfaceProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::UnixIPInterfaceProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

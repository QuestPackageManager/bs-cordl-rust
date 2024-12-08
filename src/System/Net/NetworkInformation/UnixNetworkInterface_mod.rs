#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct UnixNetworkInterface {
    __cordl_parent: crate::System::Net::NetworkInformation::NetworkInterface,
    pub ipproperties: *mut crate::System::Net::NetworkInformation::IPInterfaceProperties,
    pub name: *mut crate::System::String,
    pub addresses: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Net::IPAddress,
    >,
    pub macAddress: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _cordl_type: crate::System::Net::NetworkInformation::NetworkInterfaceType,
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::UnixNetworkInterface =>
    "System.Net.NetworkInformation"."UnixNetworkInterface"
);
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::UnixNetworkInterface {
    type Target = crate::System::Net::NetworkInformation::NetworkInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::UnixNetworkInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
impl crate::System::Net::NetworkInformation::UnixNetworkInterface {
    pub fn AddAddress(
        &mut self,
        address: *mut crate::System::Net::IPAddress,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAddress", (address))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn SetLinkLayerInfo(
        &mut self,
        index: i32,
        macAddress: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        _cordl_type: crate::System::Net::NetworkInformation::NetworkInterfaceType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLinkLayerInfo", (index, macAddress, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NetworkInterfaceType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::NetworkInformation::NetworkInterfaceType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::NetworkInformation::NetworkInterfaceType = __cordl_object
            .invoke("get_NetworkInterfaceType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::UnixNetworkInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

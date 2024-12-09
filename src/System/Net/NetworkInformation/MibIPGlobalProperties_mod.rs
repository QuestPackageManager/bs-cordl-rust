#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct MibIPGlobalProperties {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixIPGlobalProperties,
    pub StatisticsFile: *mut quest_hook::libil2cpp::Il2CppString,
    pub StatisticsFileIPv6: *mut quest_hook::libil2cpp::Il2CppString,
    pub TcpFile: *mut quest_hook::libil2cpp::Il2CppString,
    pub Tcp6File: *mut quest_hook::libil2cpp::Il2CppString,
    pub UdpFile: *mut quest_hook::libil2cpp::Il2CppString,
    pub Udp6File: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MibIPGlobalProperties =>
    "System.Net.NetworkInformation"."MibIPGlobalProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::MibIPGlobalProperties {
    type Target = crate::System::Net::NetworkInformation::UnixIPGlobalProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::MibIPGlobalProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
impl crate::System::Net::NetworkInformation::MibIPGlobalProperties {
    pub fn New(
        procDir: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (procDir))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        procDir: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (procDir))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MibIPGlobalProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::MibIPGlobalProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

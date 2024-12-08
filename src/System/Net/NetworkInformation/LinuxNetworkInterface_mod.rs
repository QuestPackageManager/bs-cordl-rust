#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct LinuxNetworkInterface {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixNetworkInterface,
    pub iface_path: *mut crate::System::String,
    pub iface_operstate_path: *mut crate::System::String,
    pub iface_flags_path: *mut crate::System::String,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::LinuxNetworkInterface =>
    "System.Net.NetworkInformation"."LinuxNetworkInterface"
);
#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::LinuxNetworkInterface {
    type Target = crate::System::Net::NetworkInformation::UnixNetworkInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::LinuxNetworkInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
impl crate::System::Net::NetworkInformation::LinuxNetworkInterface {
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
    pub fn get_IfacePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_IfacePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OperationalStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::NetworkInformation::OperationalStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::NetworkInformation::OperationalStatus = __cordl_object
            .invoke("get_OperationalStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIPProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::NetworkInformation::IPInterfaceProperties,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkInformation::IPInterfaceProperties = __cordl_object
            .invoke("GetIPProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxNetworkInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::LinuxNetworkInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

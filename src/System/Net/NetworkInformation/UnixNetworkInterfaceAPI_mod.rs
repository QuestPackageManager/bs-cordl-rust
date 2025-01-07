#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct UnixNetworkInterfaceAPI {
    __cordl_parent: crate::System::Net::NetworkInformation::NetworkInterfaceFactory,
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "UnixNetworkInterfaceAPI";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI {
    type Target = crate::System::Net::NetworkInformation::NetworkInterfaceFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
impl crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn freeifaddrs(
        ifap: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("freeifaddrs", (ifap))?;
        Ok(__cordl_ret.into())
    }
    pub fn getifaddrs(
        ifap: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getifaddrs", (ifap))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

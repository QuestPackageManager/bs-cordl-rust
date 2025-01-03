#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct AixNetworkInterfaceAPI {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI,
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixNetworkInterfaceAPI =>
    "System.Net.NetworkInformation"."AixNetworkInterfaceAPI"
);
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    type Target = crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    pub fn ByteArrayCopy(
        dst: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        elements: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ByteArrayCopy", (dst, src, elements))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllNetworkInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Net::NetworkInformation::NetworkInterface,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Net::NetworkInformation::NetworkInterface,
            >,
        > = __cordl_object.invoke("GetAllNetworkInterfaces", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn close(fd: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("close", (fd))?;
        Ok(__cordl_ret.into())
    }
    pub fn ioctl_i32_AixIoctlRequest_ByRefMut0(
        fd: i32,
        request: crate::System::Net::NetworkInformation::AixIoctlRequest,
        arg: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ioctl", (fd, request, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn ioctl_i32_AixIoctlRequest_ByRefMut1(
        fd: i32,
        request: crate::System::Net::NetworkInformation::AixIoctlRequest,
        arg: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::NetworkInformation::AixStructs::ifconf,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ioctl", (fd, request, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn ioctl_i32_AixIoctlRequest_ByRefMut2(
        fd: i32,
        request: crate::System::Net::NetworkInformation::AixIoctlRequest,
        arg: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::NetworkInformation::AixStructs::ifreq_flags,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ioctl", (fd, request, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn ioctl_i32_AixIoctlRequest_ByRefMut3(
        fd: i32,
        request: crate::System::Net::NetworkInformation::AixIoctlRequest,
        arg: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ioctl", (fd, request, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn socket(
        family: crate::System::Net::NetworkInformation::AixAddressFamily,
        _cordl_type: i32,
        protocol: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("socket", (family, _cordl_type, protocol))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

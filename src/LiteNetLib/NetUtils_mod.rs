#[cfg(feature = "LiteNetLib+NetUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct NetUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+NetUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::NetUtils => "LiteNetLib"."NetUtils"
);
#[cfg(feature = "LiteNetLib+NetUtils")]
impl std::ops::Deref for crate::LiteNetLib::NetUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl std::ops::DerefMut for crate::LiteNetLib::NetUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl crate::LiteNetLib::NetUtils {
    pub fn GetLocalIp(
        addrType: crate::LiteNetLib::LocalAddrType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalIp", (addrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalIpList_IList_1_LocalAddrType1(
        targetList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        addrType: crate::LiteNetLib::LocalAddrType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalIpList", (targetList, addrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalIpList_LocalAddrType0(
        addrType: crate::LiteNetLib::LocalAddrType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalIpList", (addrType))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeEndPoint(
        hostStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeEndPoint", (hostStr, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintInterfaceInfos() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrintInterfaceInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RelativeSequenceNumber(
        number: i32,
        expected: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RelativeSequenceNumber", (number, expected))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveAddress_AddressFamily1(
        hostStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        addressFamily: crate::System::Net::Sockets::AddressFamily,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveAddress", (hostStr, addressFamily))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveAddress_Il2CppString0(
        hostStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveAddress", (hostStr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveAddresses(
        hostStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Net::IPAddress>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Net::IPAddress>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveAddresses", (hostStr))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+NetUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::NetUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

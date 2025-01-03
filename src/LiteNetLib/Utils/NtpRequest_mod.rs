#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct NtpRequest {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _socket: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetSocket>,
    pub _onRequestComplete: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
    >,
    pub _ntpEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NtpRequest =>
    "LiteNetLib.Utils"."NtpRequest"
);
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NtpRequest {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::NtpRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl crate::LiteNetLib::Utils::NtpRequest {
    pub const DefaultPort: i32 = 123i32;
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_IPAddress_Action_1_1(
        ipAddress: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NtpRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NtpRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (ipAddress, onRequestComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_IPEndPoint_Action_1_0(
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NtpRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NtpRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (endPoint, onRequestComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Il2CppString_Action_1_3(
        ntpServerAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NtpRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NtpRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (ntpServerAddress, onRequestComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Il2CppString_i32_Action_1_2(
        ntpServerAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        port: i32,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NtpRequest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NtpRequest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (ntpServerAddress, port, onRequestComplete))?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_INetSocketListener_OnMessageReceived(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
        errorCode: crate::System::Net::Sockets::SocketError,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetSocketListener.OnMessageReceived",
                (data, length, errorCode, remoteEndPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (endPoint, onRequestComplete))?;
        Ok(__cordl_object.into())
    }
    pub fn Send(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        onRequestComplete: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::LiteNetLib::Utils::NtpPacket>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (endPoint, onRequestComplete))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::NtpRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl AsRef<crate::LiteNetLib::INetSocketListener>
for crate::LiteNetLib::Utils::NtpRequest {
    fn as_ref(&self) -> &crate::LiteNetLib::INetSocketListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl AsMut<crate::LiteNetLib::INetSocketListener>
for crate::LiteNetLib::Utils::NtpRequest {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::INetSocketListener {
        unsafe { std::mem::transmute(self) }
    }
}

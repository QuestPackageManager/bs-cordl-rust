#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct NtpRequest {
    __cordl_parent: crate::System::Object,
    pub _socket: *mut crate::LiteNetLib::NetSocket,
    pub _onRequestComplete: *mut crate::System::Action_1<
        *mut crate::LiteNetLib::Utils::NtpPacket,
    >,
    pub _ntpEndPoint: *mut crate::System::Net::IPEndPoint,
}
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NtpRequest =>
    "LiteNetLib.Utils"."NtpRequest"
);
#[cfg(feature = "LiteNetLib+Utils+NtpRequest")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NtpRequest {
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn LiteNetLib_INetSocketListener_OnMessageReceived(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
        errorCode: crate::System::Net::Sockets::SocketError,
        remoteEndPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LiteNetLib.INetSocketListener.OnMessageReceived",
                (data, length, errorCode, remoteEndPoint),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        endPoint: *mut crate::System::Net::IPEndPoint,
        onRequestComplete: *mut crate::System::Action_1<
            *mut crate::LiteNetLib::Utils::NtpPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (endPoint, onRequestComplete))?;
        Ok(__cordl_object)
    }
    pub fn Send(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
        onRequestComplete: *mut crate::System::Action_1<
            *mut crate::LiteNetLib::Utils::NtpPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (endPoint, onRequestComplete))?;
        Ok(__cordl_ret)
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
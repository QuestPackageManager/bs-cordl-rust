#[cfg(feature = "LiteNetLib+ConnectionRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionRequest {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
    pub _used: i32,
    pub Data: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    pub _Result_k__BackingField: crate::LiteNetLib::ConnectionRequestResult,
    pub ConnectionTime: i64,
    pub ConnectionNumber: u8,
    pub RemoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
}
#[cfg(feature = "LiteNetLib+ConnectionRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::ConnectionRequest => "LiteNetLib"
    ."ConnectionRequest"
);
#[cfg(feature = "LiteNetLib+ConnectionRequest")]
impl std::ops::Deref for crate::LiteNetLib::ConnectionRequest {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+ConnectionRequest")]
impl std::ops::DerefMut for crate::LiteNetLib::ConnectionRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+ConnectionRequest")]
impl crate::LiteNetLib::ConnectionRequest {
    pub fn Accept(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer> = __cordl_object
            .invoke("Accept", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AcceptIfKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetPeer> = __cordl_object
            .invoke("AcceptIfKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectionId: i64,
        connectionNumber: u8,
        netDataReader: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataReader,
        >,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectionId, connectionNumber, netDataReader, endPoint, listener),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RejectForce_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RejectForce", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RejectForce_Gc2(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RejectForce", (rejectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RejectForce_Gc3(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RejectForce", (rejectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RejectForce_Gc_i32_i32_0(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RejectForce", (rejectData, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject_Gc3(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", (rejectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject_Gc4(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", (rejectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject_Gc_i32_i32_1(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", (rejectData, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reject_Gc_i32_i32__cordl_bool0(
        &mut self,
        rejectData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        length: i32,
        force: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reject", (rejectData, start, length, force))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryActivate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryActivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRequest(
        &mut self,
        connRequest: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::NetConnectRequestPacket,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRequest", (connRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        connectionId: i64,
        connectionNumber: u8,
        netDataReader: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataReader,
        >,
        endPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        listener: quest_hook::libil2cpp::Gc<crate::LiteNetLib::NetManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (connectionId, connectionNumber, netDataReader, endPoint, listener),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LiteNetLib::ConnectionRequestResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LiteNetLib::ConnectionRequestResult = __cordl_object
            .invoke("get_Result", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Result(
        &mut self,
        value: crate::LiteNetLib::ConnectionRequestResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Result", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+ConnectionRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::ConnectionRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

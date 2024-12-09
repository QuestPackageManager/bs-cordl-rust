#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReliableHandshake {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mRecordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
    pub mHandshakeTimeout: *mut crate::Org::BouncyCastle::Crypto::Tls::Timeout,
    pub mHandshakeHash: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
    pub mCurrentInboundFlight: *mut crate::System::Collections::IDictionary,
    pub mPreviousInboundFlight: *mut crate::System::Collections::IDictionary,
    pub mOutboundFlight: *mut crate::System::Collections::IList,
    pub mResendMillis: i32,
    pub mResendTimeout: *mut crate::Org::BouncyCastle::Crypto::Tls::Timeout,
    pub mMessageSeq: i32,
    pub mNextReceiveSeq: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsReliableHandshake"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake {
    pub const InitialResendMillis: i32 = 1000i32;
    pub const MaxReceiveAhead: i32 = 16i32;
    pub const MaxResendMillis: i32 = 60000i32;
    pub const MessageHeaderLength: i32 = 12i32;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
    pub type Message = crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer"
    )]
    pub type RecordLayerBuffer = crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
    pub type Retransmit = crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit;
    pub fn BackOff(&mut self, timeoutMillis: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("BackOff", (timeoutMillis))?;
        Ok(__cordl_ret)
    }
    pub fn CheckInboundFlight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckInboundFlight", ())?;
        Ok(__cordl_ret)
    }
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPendingMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message = __cordl_object
            .invoke("GetPendingMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        timeoutMillis: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, transport, timeoutMillis))?;
        Ok(__cordl_object)
    }
    pub fn NotifyHelloComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyHelloComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn PrepareInboundFlight(
        &mut self,
        nextFlight: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareInboundFlight", (nextFlight))?;
        Ok(__cordl_ret)
    }
    pub fn PrepareToFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash = __cordl_object
            .invoke("PrepareToFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRecord(
        &mut self,
        windowSize: i32,
        epoch: i32,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRecord", (windowSize, epoch, buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message = __cordl_object
            .invoke("ReceiveMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveMessageBody(
        &mut self,
        msg_type: u8,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ReceiveMessageBody", (msg_type))?;
        Ok(__cordl_ret)
    }
    pub fn ResendOutboundFlight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResendOutboundFlight", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResetHandshakeMessagesDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetHandshakeMessagesDigest", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendMessage(
        &mut self,
        msg_type: u8,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendMessage", (msg_type, body))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateHandshakeMessagesDigest(
        &mut self,
        message: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message = __cordl_object
            .invoke("UpdateHandshakeMessagesDigest", (message))?;
        Ok(__cordl_ret)
    }
    pub fn WriteHandshakeFragment(
        &mut self,
        message: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
        fragment_offset: i32,
        fragment_length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteHandshakeFragment",
                (message, fragment_offset, fragment_length),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteMessage(
        &mut self,
        message: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteMessage", (message))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
        timeoutMillis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, transport, timeoutMillis))?;
        Ok(__cordl_ret)
    }
    pub fn get_HandshakeHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash = __cordl_object
            .invoke("get_HandshakeHash", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReliableHandshake_Message {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mMessageSeq: i32,
    pub mMsgType: u8,
    pub mBody: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsReliableHandshake/Message"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message {
    pub fn New(
        message_seq: i32,
        msg_type: u8,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message_seq, msg_type, body))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        message_seq: i32,
        msg_type: u8,
        body: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message_seq, msg_type, body))?;
        Ok(__cordl_ret)
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_Body", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Seq(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Seq", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Message")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Message {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReliableHandshake_RecordLayerBuffer {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsReliableHandshake/RecordLayerBuffer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer {
    pub fn New(_cordl_size: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn SendToRecordLayer(
        &mut self,
        recordLayer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToRecordLayer", (recordLayer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+RecordLayerBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_RecordLayerBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsReliableHandshake_Retransmit {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mOuter: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsReliableHandshake/Retransmit"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit {
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer))?;
        Ok(__cordl_object)
    }
    pub fn ReceivedHandshakeRecord(
        &mut self,
        epoch: i32,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceivedHandshakeRecord", (epoch, buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsReliableHandshake+Retransmit")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsReliableHandshake_Retransmit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

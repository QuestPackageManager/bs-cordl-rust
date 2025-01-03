#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsRecordLayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mTransport: *mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
    pub mContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    pub mPeer: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
    pub mRecordQueue: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueue,
    pub mClosed: bool,
    pub mFailed: bool,
    pub mReadVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mWriteVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mInHandshake: bool,
    pub mPlaintextLimit: i32,
    pub mCurrentEpoch: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch,
    pub mPendingEpoch: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch,
    pub mReadEpoch: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch,
    pub mWriteEpoch: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch,
    pub mRetransmit: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsHandshakeRetransmit,
    pub mRetransmitEpoch: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch,
    pub mRetransmitTimeout: *mut crate::Org::BouncyCastle::Crypto::Tls::Timeout,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer
    => "Org.BouncyCastle.Crypto.Tls"."DtlsRecordLayer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    pub const MAX_FRAGMENT_LENGTH: i32 = 16384i32;
    pub const RECORD_HEADER_LENGTH: i32 = 13i32;
    pub const RETRANSMIT_TIMEOUT: i64 = 240000i64;
    pub const TCP_MSL: i64 = 120000i64;
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
    pub fn CloseTransport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseTransport", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Fail(
        &mut self,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fail", (alertDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn Failed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Failed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMacSequenceNumber(
        epoch: i32,
        sequence_number: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMacSequenceNumber", (epoch, sequence_number))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReceiveLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetReceiveLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSendLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSendLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandshakeSuccessful(
        &mut self,
        retransmit: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsHandshakeRetransmit,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandshakeSuccessful", (retransmit))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitPendingEpoch(
        &mut self,
        pendingCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitPendingEpoch", (pendingCipher))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        transport: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        >,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        peer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsPeer>,
        contentType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (transport, context, peer, contentType))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessRecord(
        &mut self,
        received: i32,
        record: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessRecord", (received, record, buf, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn RaiseAlert(
        &mut self,
        alertLevel: u8,
        alertDescription: u8,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cause: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseAlert", (alertLevel, alertDescription, message, cause))?;
        Ok(__cordl_ret.into())
    }
    pub fn Receive(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveDatagram(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReceiveDatagram", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveRecord(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReceiveRecord", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetWriteEpoch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetWriteEpoch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendDatagram(
        sender: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        >,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendDatagram", (sender, buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendRecord(
        &mut self,
        contentType: u8,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRecord", (contentType, buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlaintextLimit(
        &mut self,
        plaintextLimit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlaintextLimit", (plaintextLimit))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWriteVersion(
        &mut self,
        writeVersion: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWriteVersion", (writeVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn Warn(
        &mut self,
        alertDescription: u8,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Warn", (alertDescription, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        transport: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        >,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        peer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsPeer>,
        contentType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (transport, context, peer, contentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsClosed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsClosed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadEpoch(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadEpoch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReadVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = __cordl_object.invoke("get_ReadVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReadVersion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadVersion", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable>
for crate::Org::BouncyCastle::Crypto::Tls::DtlsRecordLayer {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsCloseable {
        unsafe { std::mem::transmute(self) }
    }
}

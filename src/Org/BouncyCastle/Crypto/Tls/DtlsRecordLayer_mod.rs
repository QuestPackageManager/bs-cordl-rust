#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsRecordLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsRecordLayer {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn ReceiveDatagram(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReceiveDatagram", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsClosed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsClosed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandshakeSuccessful(
        &mut self,
        retransmit: *mut crate::Org::BouncyCastle::Crypto::Tls::DtlsHandshakeRetransmit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandshakeSuccessful", (retransmit))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReadVersion(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadVersion", (value))?;
        Ok(__cordl_ret)
    }
    pub fn InitPendingEpoch(
        &mut self,
        pendingCipher: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitPendingEpoch", (pendingCipher))?;
        Ok(__cordl_ret)
    }
    pub fn GetReceiveLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetReceiveLimit", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Receive(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Receive", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret)
    }
    pub fn Failed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Failed", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloseTransport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseTransport", ())?;
        Ok(__cordl_ret)
    }
    pub fn Warn(
        &mut self,
        alertDescription: u8,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Warn", (alertDescription, message))?;
        Ok(__cordl_ret)
    }
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
    pub fn RaiseAlert(
        &mut self,
        alertLevel: u8,
        alertDescription: u8,
        message: *mut crate::System::String,
        cause: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseAlert", (alertLevel, alertDescription, message, cause))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRecord(
        &mut self,
        received: i32,
        record: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessRecord", (received, record, buf, off))?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_ReadVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSendLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetSendLimit", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        peer: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
        contentType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (transport, context, peer, contentType))?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveRecord(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
        waitMillis: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReceiveRecord", (buf, off, len, waitMillis))?;
        Ok(__cordl_ret)
    }
    pub fn SetWriteVersion(
        &mut self,
        writeVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWriteVersion", (writeVersion))?;
        Ok(__cordl_ret)
    }
    pub fn ResetWriteEpoch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetWriteEpoch", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendRecord(
        &mut self,
        contentType: u8,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendRecord", (contentType, buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadEpoch(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadEpoch", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        transport: *mut crate::Org::BouncyCastle::Crypto::Tls::DatagramTransport,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        peer: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
        contentType: u8,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (transport, context, peer, contentType))?;
        Ok(__cordl_object)
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

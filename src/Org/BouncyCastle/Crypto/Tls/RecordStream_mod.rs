#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordStream {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mHandler: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
    pub mInput: *mut crate::System::IO::Stream,
    pub mOutput: *mut crate::System::IO::Stream,
    pub mPendingCompression: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCompression,
    pub mReadCompression: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCompression,
    pub mWriteCompression: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCompression,
    pub mPendingCipher: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    pub mReadCipher: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    pub mWriteCipher: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    pub mReadSeqNo: *mut crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber,
    pub mWriteSeqNo: *mut crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber,
    pub mBuffer: *mut crate::System::IO::MemoryStream,
    pub mHandshakeHash: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
    pub mHandshakeHashUpdater: *mut crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub mReadVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mWriteVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    pub mRestrictReadVersion: bool,
    pub mPlaintextLimit: i32,
    pub mCompressedLimit: i32,
    pub mCiphertextLimit: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::RecordStream =>
    "Org.BouncyCastle.Crypto.Tls"."RecordStream"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::RecordStream {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::RecordStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
impl crate::Org::BouncyCastle::Crypto::Tls::RecordStream {
    pub const DEFAULT_PLAINTEXT_LIMIT: i32 = 16384i32;
    pub const TLS_HEADER_LENGTH_OFFSET: i32 = 3i32;
    pub const TLS_HEADER_SIZE: i32 = 5i32;
    pub const TLS_HEADER_TYPE_OFFSET: i32 = 0i32;
    pub const TLS_HEADER_VERSION_OFFSET: i32 = 1i32;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream"
    )]
    pub type HandshakeHashUpdateStream = crate::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
    pub type SequenceNumber = crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber;
    pub fn CheckLength(
        length: i32,
        limit: i32,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckLength", (length, limit, alertDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckRecordHeader(
        &mut self,
        recordHeader: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRecordHeader", (recordHeader))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckType(
        _cordl_type: u8,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckType", (_cordl_type, alertDescription))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeAndVerify(
        &mut self,
        _cordl_type: u8,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("DecodeAndVerify", (_cordl_type, input, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn FinaliseHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinaliseHandshake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBufferContents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetBufferContents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlaintextLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPlaintextLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
        >,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, input, output))?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyHelloComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyHelloComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareToFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        > = __cordl_object.invoke("PrepareToFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRecord(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReadRecord", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReceivedReadCipherSpec(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceivedReadCipherSpec", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeClose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeClose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SentWriteCipherSpec(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SentWriteCipherSpec", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPendingConnectionState(
        &mut self,
        tlsCompression: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCompression,
        >,
        tlsCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPendingConnectionState", (tlsCompression, tlsCipher))?;
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
    pub fn SetRestrictReadVersion(
        &mut self,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRestrictReadVersion", (enabled))?;
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
    pub fn WriteRecord(
        &mut self,
        _cordl_type: u8,
        plaintext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        plaintextOffset: i32,
        plaintextLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteRecord",
                (_cordl_type, plaintext, plaintextOffset, plaintextLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
        >,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, input, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandshakeHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        > = __cordl_object.invoke("get_HandshakeHash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandshakeHashUpdater(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("get_HandshakeHashUpdater", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordStream_HandshakeHashUpdateStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream,
    pub mOuter: *mut crate::Org::BouncyCastle::Crypto::Tls::RecordStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream =>
    "Org.BouncyCastle.Crypto.Tls"."RecordStream/HandshakeHashUpdateStream"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream {
    type Target = crate::Org::BouncyCastle::Utilities::IO::BaseOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
impl crate::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream {
    pub fn New(
        mOuter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::RecordStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mOuter))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buf, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        mOuter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::RecordStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (mOuter))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+HandshakeHashUpdateStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_HandshakeHashUpdateStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordStream_SequenceNumber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub value: i64,
    pub exhausted: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber =>
    "Org.BouncyCastle.Crypto.Tls"."RecordStream/SequenceNumber"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
impl crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NextValue(
        &mut self,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("NextValue", (alertDescription))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+RecordStream+SequenceNumber")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::RecordStream_SequenceNumber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

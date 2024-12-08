#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsProtocol_HandshakeMessage {
    __cordl_parent: crate::System::IO::MemoryStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage =>
    "Org.BouncyCastle.Crypto.Tls"."TlsProtocol/HandshakeMessage"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage {
    type Target = crate::System::IO::MemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage {
    pub fn New_i32_1(
        handshakeType: u8,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handshakeType, length))?;
        Ok(__cordl_object)
    }
    pub fn New_u8_0(handshakeType: u8) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handshakeType))?;
        Ok(__cordl_object)
    }
    pub fn Write(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (data))?;
        Ok(__cordl_ret)
    }
    pub fn WriteToRecordStream(
        &mut self,
        protocol: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToRecordStream", (protocol))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        handshakeType: u8,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handshakeType, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_0(
        &mut self,
        handshakeType: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handshakeType))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsProtocol {
    __cordl_parent: crate::System::Object,
    pub mApplicationDataQueue: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueue,
    pub mAlertQueue: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueue,
    pub mHandshakeQueue: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueue,
    pub mRecordStream: *mut crate::Org::BouncyCastle::Crypto::Tls::RecordStream,
    pub mSecureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mTlsStream: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsStream,
    pub mClosed: bool,
    pub mFailedWithError: bool,
    pub mAppDataReady: bool,
    pub mAppDataSplitEnabled: bool,
    pub mAppDataSplitMode: i32,
    pub mExpectedVerifyData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mTlsSession: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    pub mSessionParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    pub mSecurityParameters: *mut crate::Org::BouncyCastle::Crypto::Tls::SecurityParameters,
    pub mPeerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    pub mOfferedCipherSuites: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub mOfferedCompressionMethods: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mClientExtensions: *mut crate::System::Collections::IDictionary,
    pub mServerExtensions: *mut crate::System::Collections::IDictionary,
    pub mConnectionState: i16,
    pub mResumedSession: bool,
    pub mReceivedChangeCipherSpec: bool,
    pub mSecureRenegotiation: bool,
    pub mAllowCertificateStatus: bool,
    pub mExpectSessionTicket: bool,
    pub mBlocking: bool,
    pub mInputBuffers: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueueStream,
    pub mOutputBuffer: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueueStream,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::TlsProtocol =>
    "Org.BouncyCastle.Crypto.Tls"."TlsProtocol"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol {
    pub const ADS_MODE_0_N: i16 = 1i16;
    pub const ADS_MODE_0_N_FIRSTONLY: i16 = 2i16;
    pub const ADS_MODE_1_Nsub1: i16 = 0i16;
    pub const CS_CERTIFICATE_REQUEST: i16 = 7i16;
    pub const CS_CERTIFICATE_STATUS: i16 = 5i16;
    pub const CS_CERTIFICATE_VERIFY: i16 = 12i16;
    pub const CS_CLIENT_CERTIFICATE: i16 = 10i16;
    pub const CS_CLIENT_FINISHED: i16 = 13i16;
    pub const CS_CLIENT_HELLO: i16 = 1i16;
    pub const CS_CLIENT_KEY_EXCHANGE: i16 = 11i16;
    pub const CS_CLIENT_SUPPLEMENTAL_DATA: i16 = 9i16;
    pub const CS_END: i16 = 16i16;
    pub const CS_SERVER_CERTIFICATE: i16 = 4i16;
    pub const CS_SERVER_FINISHED: i16 = 15i16;
    pub const CS_SERVER_HELLO: i16 = 2i16;
    pub const CS_SERVER_HELLO_DONE: i16 = 8i16;
    pub const CS_SERVER_KEY_EXCHANGE: i16 = 6i16;
    pub const CS_SERVER_SESSION_TICKET: i16 = 14i16;
    pub const CS_SERVER_SUPPLEMENTAL_DATA: i16 = 3i16;
    pub const CS_START: i16 = 0i16;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol+HandshakeMessage")]
    pub type HandshakeMessage = crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol_HandshakeMessage;
    pub fn ApplicationDataAvailable(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ApplicationDataAvailable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyMaxFragmentLengthExtension(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyMaxFragmentLengthExtension", ())?;
        Ok(__cordl_ret)
    }
    pub fn BlockForHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlockForHandshake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckReceivedChangeCipherSpec(
        &mut self,
        expected: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckReceivedChangeCipherSpec", (expected))?;
        Ok(__cordl_ret)
    }
    pub fn CleanupHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupHandshake", ())?;
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
    pub fn CloseInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompleteHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteHandshake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateVerifyData(
        &mut self,
        isServer: bool,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CreateVerifyData", (isServer))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAvailableInputBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetAvailableInputBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAvailableOutputBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetAvailableOutputBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleAlertMessage(
        &mut self,
        alertLevel: u8,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAlertMessage", (alertLevel, alertDescription))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAlertWarningMessage(
        &mut self,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAlertWarningMessage", (alertDescription))?;
        Ok(__cordl_ret)
    }
    pub fn HandleChangeCipherSpecMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleChangeCipherSpecMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleClose(
        &mut self,
        user_canceled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleClose", (user_canceled))?;
        Ok(__cordl_ret)
    }
    pub fn HandleException(
        &mut self,
        alertDescription: u8,
        message: *mut crate::System::String,
        cause: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleException", (alertDescription, message, cause))?;
        Ok(__cordl_ret)
    }
    pub fn HandleFailure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFailure", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleHandshakeMessage(
        &mut self,
        _cordl_type: u8,
        buf: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHandshakeMessage", (_cordl_type, buf))?;
        Ok(__cordl_ret)
    }
    pub fn InvalidateSession(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateSession", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_SecureRandom2(
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_SecureRandom0(
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Stream_SecureRandom1(
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_object)
    }
    pub fn OfferInput_Il2CppArray0(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OfferInput", (input))?;
        Ok(__cordl_ret)
    }
    pub fn OfferInput_i32_i32_1(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inputOff: i32,
        inputLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OfferInput", (input, inputOff, inputLen))?;
        Ok(__cordl_ret)
    }
    pub fn OfferOutput(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OfferOutput", (buffer, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAlertQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAlertQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessApplicationDataQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessApplicationDataQueue", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessChangeCipherSpec(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessChangeCipherSpec", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessFinishedMessage(
        &mut self,
        buf: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessFinishedMessage", (buf))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessHandshakeQueue(
        &mut self,
        queue: *mut crate::Org::BouncyCastle::Crypto::Tls::ByteQueue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessHandshakeQueue", (queue))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMaxFragmentLengthExtension(
        &mut self,
        clientExtensions: *mut crate::System::Collections::IDictionary,
        serverExtensions: *mut crate::System::Collections::IDictionary,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object
            .invoke(
                "ProcessMaxFragmentLengthExtension",
                (clientExtensions, serverExtensions, alertDescription),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRecord(
        &mut self,
        protocol: u8,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRecord", (protocol, buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseAlertFatal(
        &mut self,
        alertDescription: u8,
        message: *mut crate::System::String,
        cause: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseAlertFatal", (alertDescription, message, cause))?;
        Ok(__cordl_ret)
    }
    pub fn RaiseAlertWarning(
        &mut self,
        alertDescription: u8,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RaiseAlertWarning", (alertDescription, message))?;
        Ok(__cordl_ret)
    }
    pub fn ReadApplicationData(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadApplicationData", (buf, offset, len))?;
        Ok(__cordl_ret)
    }
    pub fn ReadInput(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadInput", (buffer, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn ReadOutput(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadOutput", (buffer, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn RefuseRenegotiation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefuseRenegotiation", ())?;
        Ok(__cordl_ret)
    }
    pub fn SafeCheckRecordHeader(
        &mut self,
        recordHeader: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeCheckRecordHeader", (recordHeader))?;
        Ok(__cordl_ret)
    }
    pub fn SafeReadRecord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeReadRecord", ())?;
        Ok(__cordl_ret)
    }
    pub fn SafeWriteRecord(
        &mut self,
        _cordl_type: u8,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeWriteRecord", (_cordl_type, buf, offset, len))?;
        Ok(__cordl_ret)
    }
    pub fn SendCertificateMessage(
        &mut self,
        certificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendCertificateMessage", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn SendChangeCipherSpecMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendChangeCipherSpecMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendFinishedMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendFinishedMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendSupplementalDataMessage(
        &mut self,
        supplementalData: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendSupplementalDataMessage", (supplementalData))?;
        Ok(__cordl_ret)
    }
    pub fn SetAppDataSplitMode(
        &mut self,
        appDataSplitMode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAppDataSplitMode", (appDataSplitMode))?;
        Ok(__cordl_ret)
    }
    pub fn WriteData(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteData", (buf, offset, len))?;
        Ok(__cordl_ret)
    }
    pub fn WriteHandshakeMessage(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteHandshakeMessage", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom2(
        &mut self,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_SecureRandom0(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Stream_SecureRandom1(
        &mut self,
        input: *mut crate::System::IO::Stream,
        output: *mut crate::System::IO::Stream,
        secureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input, output, secureRandom))?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsContext = __cordl_object
            .invoke("get_Context", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContextAdmin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsContext = __cordl_object
            .invoke("get_ContextAdmin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsClosed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsClosed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Peer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsPeer = __cordl_object
            .invoke("get_Peer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Stream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_Stream", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsProtocol")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsProtocol {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
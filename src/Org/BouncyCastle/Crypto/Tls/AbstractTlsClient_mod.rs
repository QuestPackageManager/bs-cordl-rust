#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
#[repr(C)]
#[derive(Debug)]
pub struct AbstractTlsClient {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsPeer,
    pub mCipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
    pub mContext: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClientContext,
    pub mSupportedSignatureAlgorithms: *mut crate::System::Collections::IList,
    pub mNamedCurves: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub mClientECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mServerECPointFormats: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSelectedCipherSuite: i32,
    pub mSelectedCompressionMethod: i16,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient => "Org.BouncyCastle.Crypto.Tls"
    ."AbstractTlsClient"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient {
    type Target = crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsPeer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
impl crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient {
    pub fn NotifyServerVersion(
        &mut self,
        serverVersion: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyServerVersion", (serverVersion))?;
        Ok(__cordl_ret)
    }
    pub fn AllowUnexpectedServerExtension(
        &mut self,
        extensionType: i32,
        extensionData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AllowUnexpectedServerExtension", (extensionType, extensionData))?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyExchange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsKeyExchange = __cordl_object
            .invoke("GetKeyExchange", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetClientSupplementalData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("GetClientSupplementalData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFallback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_ClientVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerSupplementalData(
        &mut self,
        serverSupplementalData: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerSupplementalData", (serverSupplementalData))?;
        Ok(__cordl_ret)
    }
    pub fn NotifySessionID(
        &mut self,
        sessionID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifySessionID", (sessionID))?;
        Ok(__cordl_ret)
    }
    pub fn GetClientExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("GetClientExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForUnexpectedServerExtension(
        &mut self,
        serverExtensions: *mut crate::System::Collections::IDictionary,
        extensionType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckForUnexpectedServerExtension",
                (serverExtensions, extensionType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        context: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsClientContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetAuthentication(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsAuthentication = __cordl_object
            .invoke("GetAuthentication", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCompression(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCompression,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCompression = __cordl_object
            .invoke("GetCompression", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifySelectedCompressionMethod(
        &mut self,
        selectedCompressionMethod: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifySelectedCompressionMethod", (selectedCompressionMethod))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessServerExtensions(
        &mut self,
        serverExtensions: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessServerExtensions", (serverExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn get_MinimumVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_MinimumVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ClientHelloRecordLayerVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion = __cordl_object
            .invoke("get_ClientHelloRecordLayerVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSessionToResume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsSession = __cordl_object
            .invoke("GetSessionToResume", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipher = __cordl_object
            .invoke("GetCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifyNewSessionTicket(
        &mut self,
        newSessionTicket: *mut crate::Org::BouncyCastle::Crypto::Tls::NewSessionTicket,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyNewSessionTicket", (newSessionTicket))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TlsCipherFactory1(
        &mut self,
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipherFactory))?;
        Ok(__cordl_ret)
    }
    pub fn GetCipherSuites(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetCipherSuites", ())?;
        Ok(__cordl_ret)
    }
    pub fn NotifySelectedCipherSuite(
        &mut self,
        selectedCipherSuite: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifySelectedCipherSuite", (selectedCipherSuite))?;
        Ok(__cordl_ret)
    }
    pub fn GetCompressionMethods(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetCompressionMethods", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_TlsCipherFactory1(
        cipherFactory: *mut crate::Org::BouncyCastle::Crypto::Tls::TlsCipherFactory,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipherFactory))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+AbstractTlsClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::AbstractTlsClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
#[repr(C)]
#[derive(Debug)]
pub struct SessionParameters_Builder {
    __cordl_parent: crate::System::Object,
    pub mCipherSuite: i32,
    pub mCompressionAlgorithm: i16,
    pub mMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPeerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    pub mPskIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSrpIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mEncodedServerExtensions: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mExtendedMasterSecret: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder =>
    "Org.BouncyCastle.Crypto.Tls"."SessionParameters/Builder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
impl crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn Validate(
        &mut self,
        condition: bool,
        parameter: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", (condition, parameter))?;
        Ok(__cordl_ret)
    }
    pub fn SetCipherSuite(
        &mut self,
        cipherSuite: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetCipherSuite", (cipherSuite))?;
        Ok(__cordl_ret)
    }
    pub fn SetServerExtensions(
        &mut self,
        serverExtensions: *mut crate::System::Collections::IDictionary,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetServerExtensions", (serverExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn SetExtendedMasterSecret(
        &mut self,
        extendedMasterSecret: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetExtendedMasterSecret", (extendedMasterSecret))?;
        Ok(__cordl_ret)
    }
    pub fn SetCompressionAlgorithm(
        &mut self,
        compressionAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetCompressionAlgorithm", (compressionAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn SetMasterSecret(
        &mut self,
        masterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetMasterSecret", (masterSecret))?;
        Ok(__cordl_ret)
    }
    pub fn SetPeerCertificate(
        &mut self,
        peerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetPeerCertificate", (peerCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn SetPskIdentity(
        &mut self,
        pskIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetPskIdentity", (pskIdentity))?;
        Ok(__cordl_ret)
    }
    pub fn SetSrpIdentity(
        &mut self,
        srpIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder = __cordl_object
            .invoke("SetSrpIdentity", (srpIdentity))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct SessionParameters {
    __cordl_parent: crate::System::Object,
    pub mCipherSuite: i32,
    pub mCompressionAlgorithm: u8,
    pub mMasterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPeerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    pub mPskIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSrpIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mEncodedServerExtensions: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mExtendedMasterSecret: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SessionParameters => "Org.BouncyCastle.Crypto.Tls"
    ."SessionParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
impl crate::Org::BouncyCastle::Crypto::Tls::SessionParameters {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters+Builder")]
    pub type Builder = crate::Org::BouncyCastle::Crypto::Tls::SessionParameters_Builder;
    pub fn get_PskIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_PskIdentity", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadServerExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke("ReadServerExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsExtendedMasterSecret(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsExtendedMasterSecret", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SrpIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_SrpIdentity", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MasterSecret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_MasterSecret", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        cipherSuite: i32,
        compressionAlgorithm: u8,
        masterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        peerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        pskIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srpIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encodedServerExtensions: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        extendedMasterSecret: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    cipherSuite,
                    compressionAlgorithm,
                    masterSecret,
                    peerCertificate,
                    pskIdentity,
                    srpIdentity,
                    encodedServerExtensions,
                    extendedMasterSecret,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CipherSuite(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CipherSuite", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PeerCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate = __cordl_object
            .invoke("get_PeerCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Tls::SessionParameters = __cordl_object
            .invoke("Copy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CompressionAlgorithm(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_CompressionAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cipherSuite: i32,
        compressionAlgorithm: u8,
        masterSecret: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        peerCertificate: *mut crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        pskIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srpIdentity: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encodedServerExtensions: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        extendedMasterSecret: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    cipherSuite,
                    compressionAlgorithm,
                    masterSecret,
                    peerCertificate,
                    pskIdentity,
                    srpIdentity,
                    encodedServerExtensions,
                    extendedMasterSecret,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SessionParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SessionParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

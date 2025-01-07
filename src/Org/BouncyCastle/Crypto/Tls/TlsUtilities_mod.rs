#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::TlsUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "TlsUtilities";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsUtilities {
    pub fn AddSignatureAlgorithmsExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddSignatureAlgorithmsExtension",
                (extensions, supportedSignatureAlgorithms),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateKeyBlock(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateKeyBlock", (context, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateKeyBlock_Ssl(
        master_secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateKeyBlock_Ssl", (master_secret, random, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMasterSecret(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        pre_master_secret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateMasterSecret", (context, pre_master_secret))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMasterSecret_Ssl(
        pre_master_secret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateMasterSecret_Ssl", (pre_master_secret, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateVerifyData(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        asciiLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handshakeHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateVerifyData", (context, asciiLabel, handshakeHash))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint16_i32_0(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint16", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint16_i64_1(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint16", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint24_i32_0(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint24", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint24_i64_1(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint24", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint32(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint32", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint48(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint48", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint64(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint64", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint8_i32_0(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint8", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckUint8_i64_1(
        i: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckUint8", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloneHash(
        hashAlgorithm: u8,
        hash: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloneHash", (hashAlgorithm, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClonePrfHash(
        prfAlgorithm: i32,
        hash: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClonePrfHash", (prfAlgorithm, hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn Concat(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Concat", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHash_SignatureAndHashAlgorithm1(
        signatureAndHashAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHash", (signatureAndHashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHash_u8_0(
        hashAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHash", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrfHash(
        prfAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrfHash", (prfAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSignatureAlgorithmsExtension(
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateSignatureAlgorithmsExtension",
                (supportedSignatureAlgorithms),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTlsSigner(
        clientCertificateType: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsSigner>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSigner,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTlsSigner", (clientCertificateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeUint8(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeUint8", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeUint8ArrayWithUint8Length(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeUint8ArrayWithUint8Length", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeOpaque8(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeOpaque8", (buf))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeSupportedSignatureAlgorithms(
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        allowAnonymous: bool,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EncodeSupportedSignatureAlgorithms",
                (supportedSignatureAlgorithms, allowAnonymous, output),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeUint16ArrayWithUint16Length(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeUint16ArrayWithUint16Length", (uints))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeUint8(
        val: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("EncodeUint8", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeUint8ArrayWithUint8Length(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeUint8ArrayWithUint8Length", (uints))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenSsl3Const() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GenSsl3Const", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllSignatureAlgorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllSignatureAlgorithms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCipherType(ciphersuite: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCipherType", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClientCertificateType(
        clientCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
        serverCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClientCertificateType", (clientCertificate, serverCertificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultDssSignatureAlgorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultDssSignatureAlgorithms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultECDsaSignatureAlgorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultECDsaSignatureAlgorithms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultRsaSignatureAlgorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultRsaSignatureAlgorithms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultSupportedSignatureAlgorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultSupportedSignatureAlgorithms", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncryptionAlgorithm(
        ciphersuite: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEncryptionAlgorithm", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtensionData(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        extensionType: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExtensionData", (extensions, extensionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashAlgorithmForPrfAlgorithm(
        prfAlgorithm: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashAlgorithmForPrfAlgorithm", (prfAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyExchangeAlgorithm(
        ciphersuite: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyExchangeAlgorithm", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMacAlgorithm(ciphersuite: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMacAlgorithm", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinimumVersion(
        ciphersuite: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinimumVersion", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOidForHashAlgorithm(
        hashAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOidForHashAlgorithm", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignatureAlgorithmsExtension(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignatureAlgorithmsExtension", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSignatureAndHashAlgorithm(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        signerCredentials: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSignerCredentials,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSignatureAndHashAlgorithm", (context, signerCredentials))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUsableSignatureAlgorithms(
        sigHashAlgs: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUsableSignatureAlgorithms", (sigHashAlgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn HMacHash(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HMacHash", (digest, secret, seed, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasExpectedEmptyExtensionData(
        extensions: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        extensionType: i32,
        alertDescription: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "HasExpectedEmptyExtensionData",
                (extensions, extensionType, alertDescription),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSigningCapability(
        clientCertificateType: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasSigningCapability", (clientCertificateType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImportSession(
        sessionID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sessionParameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SessionParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsSession>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSession,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImportSession", (sessionID, sessionParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAeadCipherSuite(ciphersuite: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAeadCipherSuite", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBlockCipherSuite(ciphersuite: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBlockCipherSuite", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSignatureAlgorithmsExtensionAllowed(
        clientVersion: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSignatureAlgorithmsExtensionAllowed", (clientVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSsl(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSsl", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsStreamCipherSuite(ciphersuite: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsStreamCipherSuite", (ciphersuite))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTimeout(
        e: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::SocketException>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTimeout", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTlsV11_ProtocolVersion0(
        version: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTlsV11", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTlsV11_TlsContext1(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTlsV11", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTlsV12_ProtocolVersion0(
        version: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTlsV12", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTlsV12_TlsContext1(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTlsV12", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCipherSuiteForSignatureAlgorithms(
        cipherSuite: i32,
        sigAlgs: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidCipherSuiteForSignatureAlgorithms", (cipherSuite, sigAlgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCipherSuiteForVersion(
        cipherSuite: i32,
        serverVersion: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidCipherSuiteForVersion", (cipherSuite, serverVersion))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint16_i32_0(i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint16", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint16_i64_1(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint16", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint24_i32_0(i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint24", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint24_i64_1(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint24", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint32(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint32", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint48(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint48", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint64(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint64", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint8_i32_0(i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint8", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUint8_i64_1(i: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidUint8", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PRF(
        context: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsContext,
        >,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        asciiLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PRF", (context, secret, asciiLabel, seed, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn PRF_legacy_Il2CppArray1(
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        labelSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PRF_legacy", (secret, label, labelSeed, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn PRF_legacy_Il2CppString0(
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        asciiLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PRF_legacy", (secret, asciiLabel, seed, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSupportedSignatureAlgorithms(
        allowAnonymous: bool,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseSupportedSignatureAlgorithms", (allowAnonymous, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllOrNothing(
        length: i32,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllOrNothing", (length, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsn1Object(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAsn1Object", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDerObject(
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadDerObject", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFully_Il2CppArray1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFully", (buf, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFully_i32_0(
        length: i32,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFully", (length, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadOpaque16(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadOpaque16", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadOpaque24(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadOpaque24", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadOpaque8(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadOpaque8", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadSignatureAlgorithmsExtension(
        extensionData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadSignatureAlgorithmsExtension", (extensionData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint16Array(
        count: i32,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint16Array", (count, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint16_Il2CppArray_i32_1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint16", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint16_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint16", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint24_Il2CppArray_i32_1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint24", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint24_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint24", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint32_Il2CppArray_i32_1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint32", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint32_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint32", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint48_Il2CppArray_i32_1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint48", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint48_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint48", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint8Array(
        count: i32,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint8Array", (count, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint8_Il2CppArray_i32_1(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint8", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUint8_Stream0(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUint8", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVersionRaw_Il2CppArray_i32_0(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVersionRaw", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVersionRaw_Stream1(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVersionRaw", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVersion_Il2CppArray_i32_0(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVersion", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVersion_Stream1(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVersion", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackHashAlgorithms(
        handshakeHash: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsHandshakeHash,
        >,
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TrackHashAlgorithms",
                (handshakeHash, supportedSignatureAlgorithms),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateKeyUsage(
        c: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
        >,
        keyUsageBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateKeyUsage", (c, keyUsageBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn VectorOfOne(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VectorOfOne", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySupportedSignatureAlgorithm(
        supportedSignatureAlgorithms: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IList,
        >,
        signatureAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SignatureAndHashAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "VerifySupportedSignatureAlgorithm",
                (supportedSignatureAlgorithms, signatureAlgorithm),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteGmtUnixTime(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteGmtUnixTime", (buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOpaque16(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteOpaque16", (buf, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOpaque24(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteOpaque24", (buf, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteOpaque8(
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteOpaque8", (buf, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16ArrayWithUint16Length_Il2CppArray_i32_1(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16ArrayWithUint16Length", (uints, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16ArrayWithUint16Length_Stream0(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16ArrayWithUint16Length", (uints, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16Array_Il2CppArray_i32_1(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16Array", (uints, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16Array_Stream0(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16Array", (uints, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16_Il2CppArray_i32_1(
        i: i32,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint16_Stream0(
        i: i32,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint16", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint24_Il2CppArray_i32_1(
        i: i32,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint24", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint24_Stream0(
        i: i32,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint24", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint32_Il2CppArray_i32_1(
        i: i64,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint32", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint32_Stream0(
        i: i64,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint32", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint48_Il2CppArray_i32_1(
        i: i64,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint48", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint48_Stream0(
        i: i64,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint48", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint64_Il2CppArray_i32_1(
        i: i64,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint64", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint64_Stream0(
        i: i64,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint64", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8ArrayWithUint8Length_Il2CppArray_i32_1(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8ArrayWithUint8Length", (uints, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8ArrayWithUint8Length_Stream0(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8ArrayWithUint8Length", (uints, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8Array_Il2CppArray_i32_1(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8Array", (uints, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8Array_Stream0(
        uints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8Array", (uints, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8_Il2CppArray_i32_1(
        i: u8,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8", (i, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUint8_Stream0(
        i: u8,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUint8", (i, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteVersion_Il2CppArray_i32_1(
        version: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteVersion", (version, buf, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteVersion_Stream0(
        version: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::ProtocolVersion,
        >,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteVersion", (version, output))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

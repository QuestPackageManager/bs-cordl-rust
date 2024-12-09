#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs10CertificationRequest {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Pkcs::CertificationRequest,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest => "Org.BouncyCastle.Pkcs"
    ."Pkcs10CertificationRequest"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest {
    type Target = crate::Org::BouncyCastle::Asn1::Pkcs::CertificationRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest {
    pub fn GetPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("GetPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (signatureFactory, subject, publicKey, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence2(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_ISignatureFactory_X509Name_AsymmetricKeyParameter_Asn1Set6(
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signatureFactory, subject, publicKey, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_ISignatureFactory_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter5(
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signingKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signatureFactory, subject, publicKey, attributes, signingKey),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoded))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter4(
        signatureAlgorithm: *mut quest_hook::libil2cpp::Il2CppString,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signingKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signatureAlgorithm, subject, publicKey, attributes, signingKey),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Stream3(
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn SetSignatureParameters(
        &mut self,
        signature: *mut crate::Org::BouncyCastle::Crypto::ISigner,
        asn1Params: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignatureParameters", (signature, asn1Params))?;
        Ok(__cordl_ret)
    }
    pub fn Verify_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", ())?;
        Ok(__cordl_ret)
    }
    pub fn Verify_AsymmetricKeyParameter1(
        &mut self,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn Verify_IVerifierFactory3(
        &mut self,
        verifier: *mut crate::Org::BouncyCastle::Crypto::IVerifierFactory,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (verifier))?;
        Ok(__cordl_ret)
    }
    pub fn Verify_IVerifierFactoryProvider2(
        &mut self,
        verifierProvider: *mut crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Verify", (verifierProvider))?;
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
    pub fn _ctor_Asn1Sequence2(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ISignatureFactory_X509Name_AsymmetricKeyParameter_Asn1Set6(
        &mut self,
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signatureFactory, subject, publicKey, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ISignatureFactory_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter5(
        &mut self,
        signatureFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signingKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signatureFactory, subject, publicKey, attributes, signingKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoded))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter4(
        &mut self,
        signatureAlgorithm: *mut quest_hook::libil2cpp::Il2CppString,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        signingKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (signatureAlgorithm, subject, publicKey, attributes, signingKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream3(
        &mut self,
        input: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

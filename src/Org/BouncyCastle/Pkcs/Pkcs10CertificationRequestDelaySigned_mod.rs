#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs10CertificationRequestDelaySigned {
    __cordl_parent: crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequestDelaySigned =>
    "Org.BouncyCastle.Pkcs"."Pkcs10CertificationRequestDelaySigned"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequestDelaySigned {
    type Target = crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequestDelaySigned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequestDelaySigned {
    pub fn GetDataToSign(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetDataToSign", ())?;
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
    pub fn New_Il2CppArray1(
        encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoded))?;
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
    pub fn New_String_X509Name_AsymmetricKeyParameter_Asn1Set5(
        signatureAlgorithm: *mut crate::System::String,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signatureAlgorithm, subject, publicKey, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_String_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter4(
        signatureAlgorithm: *mut crate::System::String,
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
    pub fn SignRequest_DerBitString1(
        &mut self,
        signedData: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SignRequest", (signedData))?;
        Ok(__cordl_ret)
    }
    pub fn SignRequest_Il2CppArray0(
        &mut self,
        signedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SignRequest", (signedData))?;
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
    pub fn _ctor_String_X509Name_AsymmetricKeyParameter_Asn1Set5(
        &mut self,
        signatureAlgorithm: *mut crate::System::String,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        publicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signatureAlgorithm, subject, publicKey, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_X509Name_AsymmetricKeyParameter_Asn1Set_AsymmetricKeyParameter4(
        &mut self,
        signatureAlgorithm: *mut crate::System::String,
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
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs10CertificationRequestDelaySigned")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs10CertificationRequestDelaySigned {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

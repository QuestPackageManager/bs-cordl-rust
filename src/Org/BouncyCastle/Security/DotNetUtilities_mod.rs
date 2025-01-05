#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct DotNetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::DotNetUtilities =>
    "Org.BouncyCastle.Security"."DotNetUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::DotNetUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::DotNetUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
impl crate::Org::BouncyCastle::Security::DotNetUtilities {
    pub fn ConvertRSAParametersField(
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertRSAParametersField", (n, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRSAProvider_CspParameters1(
        rp: crate::System::Security::Cryptography::RSAParameters,
        csp: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::CspParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateRSAProvider", (rp, csp))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRSAProvider_RSAParameters0(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateRSAProvider", (rp))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromX509Certificate(
        x509Cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromX509Certificate", (x509Cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaKeyPair_DSA0(
        dsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDsaKeyPair", (dsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaKeyPair_DSAParameters1(
        dp: crate::System::Security::Cryptography::DSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDsaKeyPair", (dp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaPublicKey_DSA0(
        dsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDsaPublicKey", (dsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaPublicKey_DSAParameters1(
        dp: crate::System::Security::Cryptography::DSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDsaPublicKey", (dp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyPair(
        privateKey: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyPair", (privateKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaKeyPair_RSA0(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRsaKeyPair", (rsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaKeyPair_RSAParameters1(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRsaKeyPair", (rp))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaPublicKey_RSA0(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRsaPublicKey", (rsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaPublicKey_RSAParameters1(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRsaPublicKey", (rp))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToRSAParameters_RsaKeyParameters0(
        rsaKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSAParameters", (rsaKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSAParameters_RsaPrivateCrtKeyParameters1(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSAParameters", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSAParameters_RsaPrivateKeyStructure2(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSAParameters", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaKeyParameters0(
        rsaKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToRSA", (rsaKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaKeyParameters_CspParameters1(
        rsaKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
        csp: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::CspParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSA", (rsaKey, csp))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateCrtKeyParameters2(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToRSA", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateCrtKeyParameters_CspParameters3(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
        csp: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::CspParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSA", (privKey, csp))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateKeyStructure4(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToRSA", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateKeyStructure_CspParameters5(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
        csp: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::CspParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToRSA", (privKey, csp))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToX509Certificate_X509Certificate1(
        x509Cert: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToX509Certificate", (x509Cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToX509Certificate_X509CertificateStructure0(
        x509Struct: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToX509Certificate", (x509Struct))?;
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
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::DotNetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

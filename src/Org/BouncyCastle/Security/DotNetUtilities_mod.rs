#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct DotNetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+DotNetUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::DotNetUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "DotNetUtilities";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::BigInteger,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("ConvertRSAParametersField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertRSAParametersField", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (n, _cordl_size)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::Security::Cryptography::RSAParameters,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::CspParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                2usize,
            >("CreateRSAProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateRSAProvider", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (rp, csp)) };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRSAProvider_RSAParameters0(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Security::Cryptography::RSAParameters),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                1usize,
            >("CreateRSAProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateRSAProvider", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (rp)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromX509Certificate(
        x509Cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
                1usize,
            >("FromX509Certificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromX509Certificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::X509Certificate,
        > = unsafe { method.invoke_unchecked((), (x509Cert)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaKeyPair_DSA0(
        dsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                1usize,
            >("GetDsaKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDsaKeyPair", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked((), (dsa)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaKeyPair_DSAParameters1(
        dp: crate::System::Security::Cryptography::DSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Security::Cryptography::DSAParameters),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                1usize,
            >("GetDsaKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDsaKeyPair", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked((), (dp)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaPublicKey_DSA0(
        dsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
                >,
                1usize,
            >("GetDsaPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDsaPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        > = unsafe { method.invoke_unchecked((), (dsa)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetDsaPublicKey_DSAParameters1(
        dp: crate::System::Security::Cryptography::DSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Security::Cryptography::DSAParameters),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
                >,
                1usize,
            >("GetDsaPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetDsaPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPublicKeyParameters,
        > = unsafe { method.invoke_unchecked((), (dp)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::AsymmetricAlgorithm,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                1usize,
            >("GetKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetKeyPair", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked((), (privateKey)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaKeyPair_RSA0(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                1usize,
            >("GetRsaKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRsaKeyPair", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked((), (rsa)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaKeyPair_RSAParameters1(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Security::Cryptography::RSAParameters),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                1usize,
            >("GetRsaKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRsaKeyPair", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked((), (rp)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaPublicKey_RSA0(
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
                >,
                1usize,
            >("GetRsaPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRsaPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        > = unsafe { method.invoke_unchecked((), (rsa)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRsaPublicKey_RSAParameters1(
        rp: crate::System::Security::Cryptography::RSAParameters,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::Security::Cryptography::RSAParameters),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
                >,
                1usize,
            >("GetRsaPublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRsaPublicKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        > = unsafe { method.invoke_unchecked((), (rp)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
                >),
                crate::System::Security::Cryptography::RSAParameters,
                1usize,
            >("ToRSAParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSAParameters", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = unsafe {
            method.invoke_unchecked((), (rsaKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToRSAParameters_RsaPrivateCrtKeyParameters1(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
                >),
                crate::System::Security::Cryptography::RSAParameters,
                1usize,
            >("ToRSAParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSAParameters", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = unsafe {
            method.invoke_unchecked((), (privKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToRSAParameters_RsaPrivateKeyStructure2(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::RSAParameters,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
                >),
                crate::System::Security::Cryptography::RSAParameters,
                1usize,
            >("ToRSAParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSAParameters", 1usize
                )
            });
        let __cordl_ret: crate::System::Security::Cryptography::RSAParameters = unsafe {
            method.invoke_unchecked((), (privKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaKeyParameters0(
        rsaKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                1usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (rsaKey)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::CspParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                2usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (rsaKey, csp)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateCrtKeyParameters2(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                1usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (privKey)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::CspParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                2usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (privKey, csp)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToRSA_RsaPrivateKeyStructure4(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
                >),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                1usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (privKey)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::CspParameters,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
                2usize,
            >("ToRSA")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToRSA", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RSA,
        > = unsafe { method.invoke_unchecked((), (privKey, csp)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                >,
                1usize,
            >("ToX509Certificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToX509Certificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = unsafe { method.invoke_unchecked((), (x509Cert)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                >,
                1usize,
            >("ToX509Certificate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToX509Certificate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = unsafe { method.invoke_unchecked((), (x509Struct)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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

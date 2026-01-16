#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsEnvelopedGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recipientInfoGenerators: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub _cordl_rand: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    pub unprotectedAttributeGenerator:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsEnvelopedGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    pub const Cast5Cbc: &'static str = "1.2.840.113533.7.66.10";
    pub const IdeaCbc: &'static str = "1.3.6.1.4.1.188.7.1.1.2";
    pub fn AddKekRecipient_Il2CppArray0(
        &mut self,
        keyAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter>,
        keyIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), quest_hook::libil2cpp::Void, 3usize>("AddKekRecipient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddKekRecipient",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (keyAlgorithm, key, keyIdentifier))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddKekRecipient_KekIdentifier1(
        &mut self,
        keyAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter>,
        kekIdentifier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("AddKekRecipient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddKekRecipient",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (keyAlgorithm, key, kekIdentifier))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddKeyAgreementRecipient(
        &mut self,
        agreementAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        senderPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        senderPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        recipientCert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
        cekWrapAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "AddKeyAgreementRecipient"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddKeyAgreementRecipient",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    agreementAlgorithm,
                    senderPrivateKey,
                    senderPublicKey,
                    recipientCert,
                    cekWrapAlgorithm,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddKeyAgreementRecipients(
        &mut self,
        agreementAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        senderPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        senderPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        recipientCerts: quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
        cekWrapAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "AddKeyAgreementRecipients"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddKeyAgreementRecipients",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    agreementAlgorithm,
                    senderPrivateKey,
                    senderPublicKey,
                    recipientCerts,
                    cekWrapAlgorithm,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddKeyTransRecipient_AsymmetricKeyParameter_Il2CppArray1(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
        subKeyId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "AddKeyTransRecipient"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddKeyTransRecipient",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pubKey, subKeyId))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddKeyTransRecipient_X509Certificate0(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::X509::X509Certificate,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AddKeyTransRecipient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddKeyTransRecipient", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cert))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddPasswordRecipient(
        &mut self,
        pbeKey: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsPbeKey>,
        kekAlgorithmOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsPbeKey>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "AddPasswordRecipient"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddPasswordRecipient",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pbeKey, kekAlgorithmOid))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddRecipientInfoGenerator(
        &mut self,
        recipientInfoGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::RecipientInfoGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::RecipientInfoGenerator,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "AddRecipientInfoGenerator"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddRecipientInfoGenerator",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (recipientInfoGenerator))? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAsn1Parameters(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encKeyBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::Asn1Encodable,
                        >,
                        2usize,
                    >("GenerateAsn1Parameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateAsn1Parameters", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable> =
            unsafe { cordl_method_info.invoke_unchecked(self, (encryptionOid, encKeyBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAlgorithmIdentifier(
        &mut self,
        encryptionOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        asn1Params: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
        cipherParameters: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::ICipherParameters,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                    >, 4usize>("GetAlgorithmIdentifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAlgorithmIdentifier",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (encryptionOid, encKey, asn1Params, cipherParameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom1(
        _cordl_rand: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_rand))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        _cordl_rand: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Security::SecureRandom,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_rand))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UnprotectedAttributeGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
                    >, 0usize>("get_UnprotectedAttributeGenerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_UnprotectedAttributeGenerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_UnprotectedAttributeGenerator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::CmsAttributeTableGenerator,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_UnprotectedAttributeGenerator"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_UnprotectedAttributeGenerator",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Cms+CmsEnvelopedGenerator")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Cms::CmsEnvelopedGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

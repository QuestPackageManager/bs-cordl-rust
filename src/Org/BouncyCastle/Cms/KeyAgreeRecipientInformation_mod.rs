#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyAgreeRecipientInformation {
    __cordl_parent: crate::Org::BouncyCastle::Cms::RecipientInformation,
    pub info: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
    >,
    pub encryptedKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "KeyAgreeRecipientInformation";
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
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    type Target = crate::Org::BouncyCastle::Cms::RecipientInformation;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    pub fn CalculateAgreedWrapKey(
        &mut self,
        wrapAlg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        senderPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        receiverPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        3usize,
                    >("CalculateAgreedWrapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CalculateAgreedWrapKey", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe {
            method
                .invoke_unchecked(self, (wrapAlg, senderPublicKey, receiverPrivateKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetContentStream(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsTypedStream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipherParameters,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Cms::CmsTypedStream,
                        >,
                        1usize,
                    >("GetContentStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetContentStream", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsTypedStream,
        > = unsafe { method.invoke_unchecked(self, (key))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPublicKeyFromOriginatorID(
        &mut self,
        origID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::OriginatorID>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Cms::OriginatorID,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        1usize,
                    >("GetPublicKeyFromOriginatorID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetPublicKeyFromOriginatorID", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked(self, (origID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPublicKeyFromOriginatorPublicKey(
        &mut self,
        receiverPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        originatorPublicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        2usize,
                    >("GetPublicKeyFromOriginatorPublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetPublicKeyFromOriginatorPublicKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe {
            method.invoke_unchecked(self, (receiverPrivateKey, originatorPublicKey))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSenderPublicKey(
        &mut self,
        receiverPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        originator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::OriginatorIdentifierOrKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Cms::OriginatorIdentifierOrKey,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        2usize,
                    >("GetSenderPublicKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSenderPublicKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked(self, (receiverPrivateKey, originator))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSessionKey(
        &mut self,
        receiverPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        1usize,
                    >("GetSessionKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSessionKey", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked(self, (receiverPrivateKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
        >,
        rid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientID>,
        encryptedKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, rid, encryptedKey, secureReadable))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadRecipientInfo(
        infos: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Cms::CmsSecureReadable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ReadRecipientInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ReadRecipientInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (infos, info, secureReadable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnwrapSessionKey(
        &mut self,
        wrapAlg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        agreedKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
                        >,
                        2usize,
                    >("UnwrapSessionKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnwrapSessionKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = unsafe { method.invoke_unchecked(self, (wrapAlg, agreedKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
        >,
        rid: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::RecipientID>,
        encryptedKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Cms::RecipientID,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::Asn1OctetString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Cms::CmsSecureReadable,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, rid, encryptedKey, secureReadable))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

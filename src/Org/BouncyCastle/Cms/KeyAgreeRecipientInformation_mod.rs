#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyAgreeRecipientInformation {
    __cordl_parent: crate::Org::BouncyCastle::Cms::RecipientInformation,
    pub info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
    pub encryptedKey: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation => "Org.BouncyCastle.Cms"
    ."KeyAgreeRecipientInformation"
);
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    type Target = crate::Org::BouncyCastle::Cms::RecipientInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyAgreeRecipientInformation")]
impl crate::Org::BouncyCastle::Cms::KeyAgreeRecipientInformation {
    pub fn CalculateAgreedWrapKey(
        &mut self,
        wrapAlg: *mut crate::System::String,
        senderPublicKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        receiverPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke(
                "CalculateAgreedWrapKey",
                (wrapAlg, senderPublicKey, receiverPrivateKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetContentStream(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::CmsTypedStream,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::CmsTypedStream = __cordl_object
            .invoke("GetContentStream", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetPublicKeyFromOriginatorID(
        &mut self,
        origID: *mut crate::Org::BouncyCastle::Cms::OriginatorID,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("GetPublicKeyFromOriginatorID", (origID))?;
        Ok(__cordl_ret)
    }
    pub fn GetPublicKeyFromOriginatorPublicKey(
        &mut self,
        receiverPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        originatorPublicKey: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorPublicKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke(
                "GetPublicKeyFromOriginatorPublicKey",
                (receiverPrivateKey, originatorPublicKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetSenderPublicKey(
        &mut self,
        receiverPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        originator: *mut crate::Org::BouncyCastle::Asn1::Cms::OriginatorIdentifierOrKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter = __cordl_object
            .invoke("GetSenderPublicKey", (receiverPrivateKey, originator))?;
        Ok(__cordl_ret)
    }
    pub fn GetSessionKey(
        &mut self,
        receiverPrivateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("GetSessionKey", (receiverPrivateKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
        rid: *mut crate::Org::BouncyCastle::Cms::RecipientID,
        encryptedKey: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, rid, encryptedKey, secureReadable))?;
        Ok(__cordl_object)
    }
    pub fn UnwrapSessionKey(
        &mut self,
        wrapAlg: *mut crate::System::String,
        agreedKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("UnwrapSessionKey", (wrapAlg, agreedKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientInfo,
        rid: *mut crate::Org::BouncyCastle::Cms::RecipientID,
        encryptedKey: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        secureReadable: *mut crate::Org::BouncyCastle::Cms::CmsSecureReadable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, rid, encryptedKey, secureReadable))?;
        Ok(__cordl_ret)
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

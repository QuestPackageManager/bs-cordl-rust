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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = __cordl_object
            .invoke(
                "CalculateAgreedWrapKey",
                (wrapAlg, senderPublicKey, receiverPrivateKey),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsTypedStream,
        > = __cordl_object.invoke("GetContentStream", (key))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = __cordl_object.invoke("GetPublicKeyFromOriginatorID", (origID))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = __cordl_object
            .invoke(
                "GetPublicKeyFromOriginatorPublicKey",
                (receiverPrivateKey, originatorPublicKey),
            )?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = __cordl_object
            .invoke("GetSenderPublicKey", (receiverPrivateKey, originator))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = __cordl_object.invoke("GetSessionKey", (receiverPrivateKey))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadRecipientInfo", (infos, info, secureReadable))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        > = __cordl_object.invoke("UnwrapSessionKey", (wrapAlg, agreedKey))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, rid, encryptedKey, secureReadable))?;
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

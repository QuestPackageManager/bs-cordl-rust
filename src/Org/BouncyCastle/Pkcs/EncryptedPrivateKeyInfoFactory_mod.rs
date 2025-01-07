#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptedPrivateKeyInfoFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Pkcs::EncryptedPrivateKeyInfoFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Pkcs";
    const CLASS_NAME: &'static str = "EncryptedPrivateKeyInfoFactory";
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
#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::EncryptedPrivateKeyInfoFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Pkcs::EncryptedPrivateKeyInfoFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
impl crate::Org::BouncyCastle::Pkcs::EncryptedPrivateKeyInfoFactory {
    pub fn CreateEncryptedPrivateKeyInfo_DerObjectIdentifier_DerObjectIdentifier_Il2CppArray_i32_SecureRandom_AsymmetricKeyParameter3(
        cipherAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        prfAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptedPrivateKeyInfo",
                (
                    cipherAlgorithm,
                    prfAlgorithm,
                    passPhrase,
                    salt,
                    iterationCount,
                    random,
                    key,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptedPrivateKeyInfo_DerObjectIdentifier_DerObjectIdentifier_Il2CppArray_i32_SecureRandom_PrivateKeyInfo4(
        cipherAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        prfAlgorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        keyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptedPrivateKeyInfo",
                (
                    cipherAlgorithm,
                    prfAlgorithm,
                    passPhrase,
                    salt,
                    iterationCount,
                    random,
                    keyInfo,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptedPrivateKeyInfo_DerObjectIdentifier_Il2CppArray_i32_AsymmetricKeyParameter0(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptedPrivateKeyInfo",
                (algorithm, passPhrase, salt, iterationCount, key),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptedPrivateKeyInfo_Il2CppString_Il2CppArray_i32_AsymmetricKeyParameter1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptedPrivateKeyInfo",
                (algorithm, passPhrase, salt, iterationCount, key),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEncryptedPrivateKeyInfo_Il2CppString_Il2CppArray_i32_PrivateKeyInfo2(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        keyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateEncryptedPrivateKeyInfo",
                (algorithm, passPhrase, salt, iterationCount, keyInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+Pkcs+EncryptedPrivateKeyInfoFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::EncryptedPrivateKeyInfoFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

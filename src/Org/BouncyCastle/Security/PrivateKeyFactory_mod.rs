#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct PrivateKeyFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::PrivateKeyFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "PrivateKeyFactory";
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
#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::PrivateKeyFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::PrivateKeyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
impl crate::Org::BouncyCastle::Security::PrivateKeyFactory {
    pub fn CreateKey_Il2CppArray0(
        privateKeyInfoData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                1usize,
            >("CreateKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked((), (privateKeyInfoData)) };
        Ok(__cordl_ret.into())
    }
    pub fn CreateKey_PrivateKeyInfo2(
        keyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                1usize,
            >("CreateKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked((), (keyInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn CreateKey_Stream1(
        inStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                1usize,
            >("CreateKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked((), (inStr)) };
        Ok(__cordl_ret.into())
    }
    pub fn DecryptKey_Asn1Object3(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        asn1Object: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                2usize,
            >("DecryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecryptKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked((), (passPhrase, asn1Object)) };
        Ok(__cordl_ret.into())
    }
    pub fn DecryptKey_EncryptedPrivateKeyInfo0(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        encInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Pkcs::EncryptedPrivateKeyInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                2usize,
            >("DecryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecryptKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe { method.invoke_unchecked((), (passPhrase, encInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn DecryptKey_Il2CppArray1(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        encryptedPrivateKeyInfoData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                2usize,
            >("DecryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecryptKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe {
            method.invoke_unchecked((), (passPhrase, encryptedPrivateKeyInfoData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecryptKey_Stream2(
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        encryptedPrivateKeyInfoStream: quest_hook::libil2cpp::Gc<
            crate::System::IO::Stream,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                >,
                2usize,
            >("DecryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecryptKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = unsafe {
            method.invoke_unchecked((), (passPhrase, encryptedPrivateKeyInfoStream))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKey_DerObjectIdentifier0(
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
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                5usize,
            >("EncryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EncryptKey", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            method
                .invoke_unchecked((), (algorithm, passPhrase, salt, iterationCount, key))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncryptKey_Il2CppString1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount: i32,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                5usize,
            >("EncryptKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EncryptKey", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe {
            method
                .invoke_unchecked((), (algorithm, passPhrase, salt, iterationCount, key))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRawKey(
        keyInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
        >,
        expectedSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("GetRawKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRawKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (keyInfo, expectedSize)) };
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
#[cfg(feature = "Org+BouncyCastle+Security+PrivateKeyFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::PrivateKeyFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

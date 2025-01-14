#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaOaepWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algId: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub engine: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "RsaOaepWrapper";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    pub fn New(
        forWrapping: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        digestOid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (forWrapping, parameters, digestOid))?;
        Ok(__cordl_object.into())
    }
    pub fn Unwrap(
        &mut self,
        cipherText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBlockResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IBlockResult,
                >,
                3usize,
            >("Unwrap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Unwrap", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockResult,
        > = unsafe { method.invoke_unchecked(self, (cipherText, offset, length)) };
        Ok(__cordl_ret.into())
    }
    pub fn Wrap(
        &mut self,
        keyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBlockResult>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IBlockResult,
                >,
                1usize,
            >("Wrap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Wrap", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockResult,
        > = unsafe { method.invoke_unchecked(self, (keyData)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        forWrapping: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        digestOid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ICipherParameters,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (forWrapping, parameters, digestOid))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                0usize,
            >("get_AlgorithmDetails")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_AlgorithmDetails", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IKeyUnwrapper>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IKeyUnwrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IKeyUnwrapper>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IKeyUnwrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IKeyWrapper>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IKeyWrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapper")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IKeyWrapper>
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapper {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IKeyWrapper {
        unsafe { std::mem::transmute(self) }
    }
}

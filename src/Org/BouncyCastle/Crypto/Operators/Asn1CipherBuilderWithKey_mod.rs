#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1CipherBuilderWithKey {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub encKey:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter>,
    pub algorithmIdentifier:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "Asn1CipherBuilderWithKey";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey {
    pub fn BuildCipher(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipher>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipher,
                        >,
                        1usize,
                    >("BuildCipher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildCipher", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipher> =
            unsafe { cordl_method_info.invoke_unchecked(self, (stream))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxOutputSize(&mut self, inputLen: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetMaxOutputSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxOutputSize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (inputLen))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        random: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID, keySize, random))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
        random: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (encryptionOID, keySize, random))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_AlgorithmDetails")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AlgorithmDetails", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipherParameters,
                        >,
                        0usize,
                    >("get_Key")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_Key",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherBuilder>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherBuilder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1CipherBuilderWithKey")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1CipherBuilderWithKey
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey {
        unsafe { std::mem::transmute(self) }
    }
}

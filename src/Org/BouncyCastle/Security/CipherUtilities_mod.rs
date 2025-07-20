#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct CipherUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::CipherUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "CipherUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::CipherUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::CipherUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl crate::Org::BouncyCastle::Security::CipherUtilities {
    #[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
    pub type CipherAlgorithm = crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm;
    #[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
    pub type CipherMode = crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode;
    #[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
    pub type CipherPadding = crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding;
    pub fn CreateBlockCipher(
        cipherAlgorithm: crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBlockCipher>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IBlockCipher,
                        >,
                        1usize,
                    >("CreateBlockCipher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateBlockCipher", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        > = unsafe { method.invoke_unchecked((), (cipherAlgorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAlgorithmName(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetAlgorithmName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetAlgorithmName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCipher_DerObjectIdentifier0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                        >,
                        1usize,
                    >("GetCipher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCipher", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = unsafe { method.invoke_unchecked((), (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCipher_Il2CppString1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBufferedCipher>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
                        >,
                        1usize,
                    >("GetCipher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCipher", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBufferedCipher,
        > = unsafe { method.invoke_unchecked((), (algorithm))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDigitIndex(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("GetDigitIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDigitIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (s))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectIdentifier(
        mechanism: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >,
                        1usize,
                    >("GetObjectIdentifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetObjectIdentifier", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = unsafe { method.invoke_unchecked((), (mechanism))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Algorithms() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ICollection>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::ICollection,
                        >,
                        0usize,
                    >("get_Algorithms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Algorithms", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ICollection,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::CipherUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherUtilities_CipherAlgorithm {
    #[default]
    AES = 0i32,
    ARC4 = 1i32,
    BLOWFISH = 2i32,
    CAMELLIA = 3i32,
    CAST5 = 4i32,
    CAST6 = 5i32,
    CHACHA = 6i32,
    CHACHA20_POLY1305 = 7i32,
    CHACHA7539 = 8i32,
    DES = 9i32,
    DESEDE = 10i32,
    ELGAMAL = 11i32,
    GOST28147 = 12i32,
    HC128 = 13i32,
    HC256 = 14i32,
    IDEA = 15i32,
    NOEKEON = 16i32,
    PBEWITHSHAAND128BITRC4 = 17i32,
    PBEWITHSHAAND40BITRC4 = 18i32,
    RC2 = 19i32,
    RC5 = 20i32,
    RC5_64 = 21i32,
    RC6 = 22i32,
    RIJNDAEL = 23i32,
    RSA = 24i32,
    SALSA20 = 25i32,
    SEED = 26i32,
    SERPENT = 27i32,
    SKIPJACK = 28i32,
    SM4 = 29i32,
    TEA = 30i32,
    THREEFISH_1024 = 33i32,
    THREEFISH_256 = 31i32,
    THREEFISH_512 = 32i32,
    TNEPRES = 34i32,
    TWOFISH = 35i32,
    VMPC = 36i32,
    VMPC_KSA3 = 37i32,
    XTEA = 38i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "CipherUtilities/CipherAlgorithm";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherAlgorithm")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherUtilities_CipherMode {
    #[default]
    CBC = 2i32,
    CCM = 3i32,
    CFB = 4i32,
    CTR = 5i32,
    CTS = 6i32,
    EAX = 7i32,
    ECB = 0i32,
    GCM = 8i32,
    GOFB = 9i32,
    NONE = 1i32,
    OCB = 10i32,
    OFB = 11i32,
    OPENPGPCFB = 12i32,
    SIC = 13i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "CipherUtilities/CipherMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherUtilities_CipherPadding {
    #[default]
    ISO10126D2PADDING = 3i32,
    ISO10126PADDING = 2i32,
    ISO10126_2PADDING = 4i32,
    ISO7816_4PADDING = 5i32,
    ISO9796_1 = 7i32,
    ISO9796_1PADDING = 8i32,
    ISO9797_1PADDING = 6i32,
    NOPADDING = 0i32,
    OAEP = 9i32,
    OAEPPADDING = 10i32,
    OAEPWITHMD5ANDMGF1PADDING = 11i32,
    OAEPWITHSHA1ANDMGF1PADDING = 12i32,
    OAEPWITHSHA224ANDMGF1PADDING = 14i32,
    OAEPWITHSHA256ANDMGF1PADDING = 16i32,
    OAEPWITHSHA384ANDMGF1PADDING = 18i32,
    OAEPWITHSHA512ANDMGF1PADDING = 20i32,
    OAEPWITHSHA_1ANDMGF1PADDING = 13i32,
    OAEPWITHSHA_224ANDMGF1PADDING = 15i32,
    OAEPWITHSHA_256ANDMGF1PADDING = 17i32,
    OAEPWITHSHA_384ANDMGF1PADDING = 19i32,
    OAEPWITHSHA_512ANDMGF1PADDING = 21i32,
    PKCS1 = 22i32,
    PKCS1PADDING = 23i32,
    PKCS5 = 24i32,
    PKCS5PADDING = 25i32,
    PKCS7 = 26i32,
    PKCS7PADDING = 27i32,
    RAW = 1i32,
    TBCPADDING = 28i32,
    WITHCTS = 29i32,
    X923PADDING = 30i32,
    ZEROBYTEPADDING = 31i32,
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "CipherUtilities/CipherPadding";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Org::BouncyCastle::Security::CipherUtilities_CipherPadding {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

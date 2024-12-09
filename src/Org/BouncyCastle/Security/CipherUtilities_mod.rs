#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct CipherUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Security::CipherUtilities =>
    "Org.BouncyCastle.Security"."CipherUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::CipherUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::CipherUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherUtilities_CipherAlgorithm {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::CipherUtilities_CipherAlgorithm =>
    "Org.BouncyCastle.Security"."CipherUtilities/CipherAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherUtilities_CipherMode {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::CipherUtilities_CipherMode =>
    "Org.BouncyCastle.Security"."CipherUtilities/CipherMode"
);
#[cfg(feature = "Org+BouncyCastle+Security+CipherUtilities+CipherPadding")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherUtilities_CipherPadding {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Security::CipherUtilities_CipherPadding =>
    "Org.BouncyCastle.Security"."CipherUtilities/CipherPadding"
);

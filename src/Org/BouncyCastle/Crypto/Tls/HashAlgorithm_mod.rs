#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
#[repr(C)]
#[derive(Debug)]
pub struct HashAlgorithm {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::HashAlgorithm =>
    "Org.BouncyCastle.Crypto.Tls"."HashAlgorithm"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::HashAlgorithm {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::HashAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
impl crate::Org::BouncyCastle::Crypto::Tls::HashAlgorithm {
    pub const md5: u8 = 1u8;
    pub const none: u8 = 0u8;
    pub const sha1: u8 = 2u8;
    pub const sha224: u8 = 3u8;
    pub const sha256: u8 = 4u8;
    pub const sha384: u8 = 5u8;
    pub const sha512: u8 = 6u8;
    pub fn GetName(
        hashAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetName", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetText(
        hashAlgorithm: u8,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetText", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrivate(hashAlgorithm: u8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrivate", (hashAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRecognized(hashAlgorithm: u8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsRecognized", (hashAlgorithm))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+HashAlgorithm")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::HashAlgorithm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

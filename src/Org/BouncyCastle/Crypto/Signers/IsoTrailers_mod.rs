#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
#[repr(C)]
#[derive(Debug)]
pub struct IsoTrailers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Signers::IsoTrailers
    => "Org.BouncyCastle.Crypto.Signers"."IsoTrailers"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::IsoTrailers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::IsoTrailers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
impl crate::Org::BouncyCastle::Crypto::Signers::IsoTrailers {
    pub const TRAILER_IMPLICIT: i32 = 188i32;
    pub const TRAILER_RIPEMD128: i32 = 13004i32;
    pub const TRAILER_RIPEMD160: i32 = 12748i32;
    pub const TRAILER_SHA1: i32 = 13260i32;
    pub const TRAILER_SHA224: i32 = 14540i32;
    pub const TRAILER_SHA256: i32 = 13516i32;
    pub const TRAILER_SHA384: i32 = 14028i32;
    pub const TRAILER_SHA512: i32 = 13772i32;
    pub const TRAILER_SHA512_224: i32 = 14796i32;
    pub const TRAILER_SHA512_256: i32 = 16588i32;
    pub const TRAILER_WHIRLPOOL: i32 = 14284i32;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IsoTrailers")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::IsoTrailers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

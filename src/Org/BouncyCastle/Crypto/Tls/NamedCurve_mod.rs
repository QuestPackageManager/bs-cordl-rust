#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedCurve {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::NamedCurve =>
    "Org.BouncyCastle.Crypto.Tls"."NamedCurve"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    pub const arbitrary_explicit_char2_curves: i32 = 65282i32;
    pub const arbitrary_explicit_prime_curves: i32 = 65281i32;
    pub const brainpoolP256r1: i32 = 26i32;
    pub const brainpoolP384r1: i32 = 27i32;
    pub const brainpoolP512r1: i32 = 28i32;
    pub const secp160k1: i32 = 15i32;
    pub const secp160r1: i32 = 16i32;
    pub const secp160r2: i32 = 17i32;
    pub const secp192k1: i32 = 18i32;
    pub const secp192r1: i32 = 19i32;
    pub const secp224k1: i32 = 20i32;
    pub const secp224r1: i32 = 21i32;
    pub const secp256k1: i32 = 22i32;
    pub const secp256r1: i32 = 23i32;
    pub const secp384r1: i32 = 24i32;
    pub const secp521r1: i32 = 25i32;
    pub const sect163k1: i32 = 1i32;
    pub const sect163r1: i32 = 2i32;
    pub const sect163r2: i32 = 3i32;
    pub const sect193r1: i32 = 4i32;
    pub const sect193r2: i32 = 5i32;
    pub const sect233k1: i32 = 6i32;
    pub const sect233r1: i32 = 7i32;
    pub const sect239k1: i32 = 8i32;
    pub const sect283k1: i32 = 9i32;
    pub const sect283r1: i32 = 10i32;
    pub const sect409k1: i32 = 11i32;
    pub const sect409r1: i32 = 12i32;
    pub const sect571k1: i32 = 13i32;
    pub const sect571r1: i32 = 14i32;
    pub fn IsValid(namedCurve: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (namedCurve))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefersToASpecificNamedCurve(
        namedCurve: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RefersToASpecificNamedCurve", (namedCurve))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

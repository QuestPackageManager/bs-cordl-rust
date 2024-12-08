#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct Poly1305KeyGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::CipherKeyGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::Poly1305KeyGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."Poly1305KeyGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::Poly1305KeyGenerator {
    type Target = crate::Org::BouncyCastle::Crypto::CipherKeyGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::Poly1305KeyGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::Poly1305KeyGenerator {
    pub const R_MASK_HIGH_4: u8 = 15u8;
    pub const R_MASK_LOW_2: u8 = 252u8;
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
    pub fn engineGenerateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("engineGenerateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn engineInit(
        &mut self,
        param: *mut crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("engineInit", (param))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+Poly1305KeyGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::Poly1305KeyGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

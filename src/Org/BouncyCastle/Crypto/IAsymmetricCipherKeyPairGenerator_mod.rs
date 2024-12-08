#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct IAsymmetricCipherKeyPairGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator =>
    "Org.BouncyCastle.Crypto"."IAsymmetricCipherKeyPairGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
impl crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
    pub fn Init(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair = __cordl_object
            .invoke("GenerateKeyPair", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IAsymmetricCipherKeyPairGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

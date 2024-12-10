#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct AsymmetricCipherKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub publicParameter: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub privateParameter: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair => "Org.BouncyCastle.Crypto"
    ."AsymmetricCipherKeyPair"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
impl crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair {
    pub fn New(
        publicParameter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privateParameter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicParameter, privateParameter))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        publicParameter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        privateParameter: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicParameter, privateParameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Private(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = __cordl_object.invoke("get_Private", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Public(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        > = __cordl_object.invoke("get_Public", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+AsymmetricCipherKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

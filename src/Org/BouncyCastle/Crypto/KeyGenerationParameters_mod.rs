#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyGenerationParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub strength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::KeyGenerationParameters => "Org.BouncyCastle.Crypto"
    ."KeyGenerationParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::KeyGenerationParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::KeyGenerationParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
impl crate::Org::BouncyCastle::Crypto::KeyGenerationParameters {
    pub fn New(
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        strength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (random, strength))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        strength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (random, strength))?;
        Ok(__cordl_ret)
    }
    pub fn get_Random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Security::SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Security::SecureRandom = __cordl_object
            .invoke("get_Random", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Strength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Strength", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+KeyGenerationParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::KeyGenerationParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

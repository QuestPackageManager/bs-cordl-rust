#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IDrbgProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Prng::IDrbgProvider
    => "Org.BouncyCastle.Crypto.Prng"."IDrbgProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg = __cordl_object
            .invoke("Get", (entropySource))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+IDrbgProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

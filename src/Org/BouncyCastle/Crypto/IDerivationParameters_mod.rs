#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct IDerivationParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::IDerivationParameters
    => "Org.BouncyCastle.Crypto"."IDerivationParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IDerivationParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IDerivationParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
impl crate::Org::BouncyCastle::Crypto::IDerivationParameters {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDerivationParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::IDerivationParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

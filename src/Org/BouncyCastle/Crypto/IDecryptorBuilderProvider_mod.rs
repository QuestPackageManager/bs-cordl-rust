#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IDecryptorBuilderProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider => "Org.BouncyCastle.Crypto"
    ."IDecryptorBuilderProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
impl crate::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider {
    pub fn CreateDecryptorBuilder(
        &mut self,
        algorithmDetails: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherBuilder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilder,
        > = __cordl_object.invoke("CreateDecryptorBuilder", (algorithmDetails))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDecryptorBuilderProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::IDecryptorBuilderProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct IesWithCipherParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::IesParameters,
    pub cipherKeySize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::IesWithCipherParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."IesWithCipherParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::IesWithCipherParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::IesParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::IesWithCipherParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::IesWithCipherParameters {
    pub fn _ctor(
        &mut self,
        derivation: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        macKeySize: i32,
        cipherKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (derivation, encoding, macKeySize, cipherKeySize))?;
        Ok(__cordl_ret)
    }
    pub fn get_CipherKeySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CipherKeySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        derivation: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        encoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        macKeySize: i32,
        cipherKeySize: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (derivation, encoding, macKeySize, cipherKeySize))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+IesWithCipherParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::IesWithCipherParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

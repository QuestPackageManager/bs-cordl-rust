#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DesParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::DesParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."DesParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::DesParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::DesParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::DesParameters {
    pub const DesKeyLength: i32 = 8i32;
    pub const N_DES_WEAK_KEYS: i32 = 16i32;
    pub fn New_Il2CppArray0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, keyOff, keyLen))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, keyOff, keyLen))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::DesParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

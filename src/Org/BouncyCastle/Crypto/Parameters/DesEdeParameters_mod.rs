#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DesEdeParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::DesParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::DesEdeParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."DesEdeParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::DesEdeParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::DesParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::DesEdeParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::DesEdeParameters {
    pub const DesEdeKeyLength: i32 = 24i32;
    pub fn _ctor_Il2CppArray0(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, keyOff, keyLen))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_1(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, keyOff, keyLen))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DesEdeParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::DesEdeParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

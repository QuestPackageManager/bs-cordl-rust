#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct RC2Parameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub bits: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::RC2Parameters =>
    "Org.BouncyCastle.Crypto.Parameters"."RC2Parameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::RC2Parameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::RC2Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::RC2Parameters {
    pub fn New_Il2CppArray0(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_2(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, bits))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_1(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, keyOff, keyLen))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_3(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, keyOff, keyLen, bits))?;
        Ok(__cordl_object)
    }
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
    pub fn _ctor_i32_2(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, bits))?;
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
    pub fn _ctor_i32_i32_i32_3(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        keyLen: i32,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, keyOff, keyLen, bits))?;
        Ok(__cordl_ret)
    }
    pub fn get_EffectiveKeyBits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_EffectiveKeyBits", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC2Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::RC2Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

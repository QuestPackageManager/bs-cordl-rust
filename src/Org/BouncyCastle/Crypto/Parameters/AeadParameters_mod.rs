#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct AeadParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub associatedText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub macSize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::AeadParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."AeadParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::AeadParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::AeadParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::AeadParameters {
    pub fn GetAssociatedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetAssociatedText", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetNonce", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray1(
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        macSize: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        associatedText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, macSize, nonce, associatedText))?;
        Ok(__cordl_object)
    }
    pub fn New_KeyParameter_i32_Il2CppArray0(
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        macSize: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, macSize, nonce))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        macSize: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        associatedText: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, macSize, nonce, associatedText))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_KeyParameter_i32_Il2CppArray0(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        macSize: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, macSize, nonce))?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("get_Key", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MacSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MacSize", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+AeadParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::AeadParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

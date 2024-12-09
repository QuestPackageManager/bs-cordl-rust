#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct Rfc3394WrapEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub param: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub forWrapping: bool,
    pub iv: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine =>
    "Org.BouncyCastle.Crypto.Engines"."Rfc3394WrapEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine {
    pub fn Init(
        &mut self,
        forWrapping: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forWrapping, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine))?;
        Ok(__cordl_object)
    }
    pub fn Unwrap(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Unwrap", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn Wrap(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Wrap", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Rfc3394WrapEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::Rfc3394WrapEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

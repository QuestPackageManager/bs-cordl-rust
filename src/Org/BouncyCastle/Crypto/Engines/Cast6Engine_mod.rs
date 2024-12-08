#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
#[repr(C)]
#[derive(Debug)]
pub struct Cast6Engine {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Engines::Cast5Engine,
    pub _Kr: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _Km: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub _Tr: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _Tm: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub _workingKey: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Engines::Cast6Engine
    => "Org.BouncyCastle.Crypto.Engines"."Cast6Engine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::Cast6Engine {
    type Target = crate::Org::BouncyCastle::Crypto::Engines::Cast5Engine;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::Cast6Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
impl crate::Org::BouncyCastle::Crypto::Engines::Cast6Engine {
    pub const BLOCK_SIZE: i32 = 16i32;
    pub const ROUNDS: i32 = 12i32;
    pub fn CAST_Decipher(
        &mut self,
        A: u32,
        B: u32,
        C: u32,
        D: u32,
        result: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CAST_Decipher", (A, B, C, D, result))?;
        Ok(__cordl_ret)
    }
    pub fn CAST_Encipher(
        &mut self,
        A: u32,
        B: u32,
        C: u32,
        D: u32,
        result: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CAST_Encipher", (A, B, C, D, result))?;
        Ok(__cordl_ret)
    }
    pub fn DecryptBlock(
        &mut self,
        src: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srcIndex: i32,
        dst: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("DecryptBlock", (src, srcIndex, dst, dstIndex))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptBlock(
        &mut self,
        src: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srcIndex: i32,
        dst: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("EncryptBlock", (src, srcIndex, dst, dstIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+Cast6Engine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::Cast6Engine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

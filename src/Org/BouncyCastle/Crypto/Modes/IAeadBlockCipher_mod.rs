#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct IAeadBlockCipher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher =>
    "Org.BouncyCastle.Crypto.Modes"."IAeadBlockCipher"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
impl crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher {
    pub fn GetBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnderlyingCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher = __cordl_object
            .invoke("GetUnderlyingCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+IAeadBlockCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Modes::IAeadBlockCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
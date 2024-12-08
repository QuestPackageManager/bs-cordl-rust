#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct RC5Parameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    pub rounds: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::RC5Parameters =>
    "Org.BouncyCastle.Crypto.Parameters"."RC5Parameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::RC5Parameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Parameters::RC5Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::RC5Parameters {
    pub fn _ctor(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rounds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, rounds))?;
        Ok(__cordl_ret)
    }
    pub fn get_Rounds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Rounds", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        rounds: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, rounds))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RC5Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::RC5Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

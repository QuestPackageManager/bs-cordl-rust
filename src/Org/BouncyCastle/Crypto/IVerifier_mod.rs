#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
#[repr(C)]
#[derive(Debug)]
pub struct IVerifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::IVerifier =>
    "Org.BouncyCastle.Crypto"."IVerifier"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IVerifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IVerifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
impl crate::Org::BouncyCastle::Crypto::IVerifier {
    pub fn IsVerified_Il2CppArray0(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsVerified", (data))?;
        Ok(__cordl_ret)
    }
    pub fn IsVerified_i32_i32_1(
        &mut self,
        source: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsVerified", (source, off, length))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IVerifier")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crypto::IVerifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

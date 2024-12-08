#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
#[repr(C)]
#[derive(Debug)]
pub struct ISignerWithRecovery {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::ISignerWithRecovery
    => "Org.BouncyCastle.Crypto"."ISignerWithRecovery"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
impl crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
    pub fn HasFullMessage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFullMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateWithRecoveredMessage(
        &mut self,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWithRecoveredMessage", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn GetRecoveredMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetRecoveredMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+ISignerWithRecovery")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

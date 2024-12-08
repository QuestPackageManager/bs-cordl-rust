#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
#[repr(C)]
#[derive(Debug)]
pub struct IEncryptedValuePadder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::IEncryptedValuePadder
    => "Org.BouncyCastle.Crmf"."IEncryptedValuePadder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
impl crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder {
    pub fn GetPaddedData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPaddedData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnpaddedData(
        &mut self,
        paddedData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetUnpaddedData", (paddedData))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+IEncryptedValuePadder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::IEncryptedValuePadder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

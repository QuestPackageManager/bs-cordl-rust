#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
#[repr(C)]
#[derive(Debug)]
pub struct RevocationReason {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SignatureSubpacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::RevocationReason =>
    "Org.BouncyCastle.Bcpg"."RevocationReason"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::RevocationReason {
    type Target = crate::Org::BouncyCastle::Bcpg::SignatureSubpacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::RevocationReason {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
impl crate::Org::BouncyCastle::Bcpg::RevocationReason {
    pub fn GetRevocationDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetRevocationDescription", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRevocationReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::RevocationReasonTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::RevocationReasonTag = __cordl_object
            .invoke("GetRevocationReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_RevocationReasonTag_Il2CppString1(
        isCritical: bool,
        reason: crate::Org::BouncyCastle::Bcpg::RevocationReasonTag,
        description: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isCritical, reason, description))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool_Il2CppArray0(
        isCritical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (isCritical, isLongLength, data))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_RevocationReasonTag_Il2CppString1(
        &mut self,
        isCritical: bool,
        reason: crate::Org::BouncyCastle::Bcpg::RevocationReasonTag,
        description: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isCritical, reason, description))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool_Il2CppArray0(
        &mut self,
        isCritical: bool,
        isLongLength: bool,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (isCritical, isLongLength, data))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+RevocationReason")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::RevocationReason {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

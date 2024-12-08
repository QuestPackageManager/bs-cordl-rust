#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
#[repr(C)]
#[derive(Debug)]
pub struct IX509Extension {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::IX509Extension =>
    "Org.BouncyCastle.X509"."IX509Extension"
);
#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::IX509Extension {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::IX509Extension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
impl crate::Org::BouncyCastle::X509::IX509Extension {
    pub fn GetCriticalExtensionOids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetCriticalExtensionOids", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExtensionValue_DerObjectIdentifier1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("GetExtensionValue", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtensionValue_String0(
        &mut self,
        oid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("GetExtensionValue", (oid))?;
        Ok(__cordl_ret)
    }
    pub fn GetNonCriticalExtensionOids(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetNonCriticalExtensionOids", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+IX509Extension")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::IX509Extension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
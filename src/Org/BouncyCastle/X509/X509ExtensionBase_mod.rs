#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ExtensionBase {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509ExtensionBase =>
    "Org.BouncyCastle.X509"."X509ExtensionBase"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509ExtensionBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509ExtensionBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
impl crate::Org::BouncyCastle::X509::X509ExtensionBase {
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
    pub fn GetExtensionOids(
        &mut self,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetExtensionOids", (critical))?;
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
    pub fn GetX509Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("GetX509Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "Org+BouncyCastle+X509+X509ExtensionBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509ExtensionBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
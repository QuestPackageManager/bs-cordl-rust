#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyAgreeRecipientIdentifier {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub issuerSerial: *mut crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
    pub rKeyID: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientIdentifier =>
    "Org.BouncyCastle.Asn1.Cms"."KeyAgreeRecipientIdentifier"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientIdentifier {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
impl crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientIdentifier {
    pub fn _ctor_IssuerAndSerialNumber0(
        &mut self,
        issuerSerial: *mut crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerSerial))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RecipientKeyIdentifier1(
        &mut self,
        rKeyID: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rKeyID))?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerAndSerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber = __cordl_object
            .invoke("get_IssuerAndSerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RKeyID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier = __cordl_object
            .invoke("get_RKeyID", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IssuerAndSerialNumber0(
        issuerSerial: *mut crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerSerial))?;
        Ok(__cordl_object)
    }
    pub fn New_RecipientKeyIdentifier1(
        rKeyID: *mut crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rKeyID))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KeyAgreeRecipientIdentifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::KeyAgreeRecipientIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

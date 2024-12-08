#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
#[repr(C)]
#[derive(Debug)]
pub struct Attributes {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::Attributes =>
    "Org.BouncyCastle.Asn1.Cms"."Attributes"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::Attributes {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
impl crate::Org::BouncyCastle::Asn1::Cms::Attributes {
    pub fn GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cms::Attribute,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Cms::Attribute,
        > = __cordl_object.invoke("GetAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Set0(
        &mut self,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1EncodableVector1(
        &mut self,
        v: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
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
    pub fn New_Asn1Set0(
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1EncodableVector1(
        v: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+Attributes")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::Attributes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

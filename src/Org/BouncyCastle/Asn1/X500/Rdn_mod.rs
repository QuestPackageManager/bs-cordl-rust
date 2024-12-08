#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
#[repr(C)]
#[derive(Debug)]
pub struct Rdn {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub values: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::X500::Rdn =>
    "Org.BouncyCastle.Asn1.X500"."Rdn"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X500::Rdn {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X500::Rdn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
impl crate::Org::BouncyCastle::Asn1::X500::Rdn {
    pub fn GetTypesAndValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        > = __cordl_object.invoke("GetTypesAndValues", ())?;
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
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Set0(
        &mut self,
        values: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (values))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_Asn1Encodable1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        value: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AttributeTypeAndValue2(
        &mut self,
        attrTAndV: *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (attrTAndV))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray3(
        &mut self,
        aAndVs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aAndVs))?;
        Ok(__cordl_ret)
    }
    pub fn GetFirst(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue = __cordl_object
            .invoke("GetFirst", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsMultiValued(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMultiValued", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Set0(
        values: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (values))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_Asn1Encodable1(
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        value: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, value))?;
        Ok(__cordl_object)
    }
    pub fn New_AttributeTypeAndValue2(
        attrTAndV: *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (attrTAndV))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray3(
        aAndVs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X500::AttributeTypeAndValue,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aAndVs))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X500+Rdn")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Asn1::X500::Rdn {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

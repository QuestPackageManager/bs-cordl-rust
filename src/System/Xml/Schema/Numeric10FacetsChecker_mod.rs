#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct Numeric10FacetsChecker {
    __cordl_parent: crate::System::Xml::Schema::FacetsChecker,
    pub maxValue: crate::System::Decimal,
    pub minValue: crate::System::Decimal,
}
#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::Numeric10FacetsChecker =>
    "System.Xml.Schema"."Numeric10FacetsChecker"
);
#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
impl std::ops::Deref for crate::System::Xml::Schema::Numeric10FacetsChecker {
    type Target = crate::System::Xml::Schema::FacetsChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Numeric10FacetsChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
impl crate::System::Xml::Schema::Numeric10FacetsChecker {
    pub fn CheckTotalAndFractionDigits(
        &mut self,
        value: crate::System::Decimal,
        totalDigits: i32,
        fractionDigits: i32,
        checkTotal: bool,
        checkFraction: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke(
                "CheckTotalAndFractionDigits",
                (value, totalDigits, fractionDigits, checkTotal, checkFraction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Decimal1(
        &mut self,
        value: crate::System::Decimal,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_Object0(
        &mut self,
        value: *mut crate::System::Object,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_i16_4(
        &mut self,
        value: i16,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_i32_3(
        &mut self,
        value: i32,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_i64_2(
        &mut self,
        value: i64,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn MatchEnumeration_Decimal_XmlValueConverter1(
        &mut self,
        value: crate::System::Decimal,
        enumeration: *mut crate::System::Collections::ArrayList,
        valueConverter: *mut crate::System::Xml::Schema::XmlValueConverter,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchEnumeration", (value, enumeration, valueConverter))?;
        Ok(__cordl_ret)
    }
    pub fn MatchEnumeration_Object_XmlSchemaDatatype0(
        &mut self,
        value: *mut crate::System::Object,
        enumeration: *mut crate::System::Collections::ArrayList,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchEnumeration", (value, enumeration, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        minVal: crate::System::Decimal,
        maxVal: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (minVal, maxVal))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        minVal: crate::System::Decimal,
        maxVal: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (minVal, maxVal))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+Numeric10FacetsChecker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::Numeric10FacetsChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

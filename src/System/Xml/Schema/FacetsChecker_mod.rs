#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
#[repr(C)]
#[derive(Debug)]
pub struct FacetsChecker {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::FacetsChecker =>
    "System.Xml.Schema"."FacetsChecker"
);
#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
impl std::ops::Deref for crate::System::Xml::Schema::FacetsChecker {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
impl std::ops::DerefMut for crate::System::Xml::Schema::FacetsChecker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
impl crate::System::Xml::Schema::FacetsChecker {
    #[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler")]
    pub type FacetsCompiler = crate::System::Xml::Schema::FacetsChecker_FacetsCompiler;
    pub fn CheckLexicalFacets(
        &mut self,
        parseString: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckLexicalFacets", (parseString, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckPatternFacets(
        &mut self,
        restriction: *mut crate::System::Xml::Schema::RestrictionFacets,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckPatternFacets", (restriction, value))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_DateTime5(
        &mut self,
        value: crate::System::DateTime,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
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
    pub fn CheckValueFacets_Il2CppArray9(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
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
    pub fn CheckValueFacets_String8(
        &mut self,
        value: *mut crate::System::String,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_TimeSpan10(
        &mut self,
        value: crate::System::TimeSpan,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_XmlQualifiedName11(
        &mut self,
        value: *mut crate::System::Xml::XmlQualifiedName,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_f32_7(
        &mut self,
        value: f32,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CheckValueFacets", (value, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValueFacets_f64_6(
        &mut self,
        value: f64,
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
    pub fn CheckWhitespaceFacets(
        &mut self,
        s: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckWhitespaceFacets", (s, datatype))?;
        Ok(__cordl_ret)
    }
    pub fn ConstructRestriction(
        &mut self,
        datatype: *mut crate::System::Xml::Schema::DatatypeImplementation,
        facets: *mut crate::System::Xml::Schema::XmlSchemaObjectCollection,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::RestrictionFacets,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::RestrictionFacets = __cordl_object
            .invoke("ConstructRestriction", (datatype, facets, nameTable))?;
        Ok(__cordl_ret)
    }
    pub fn MatchEnumeration(
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
#[cfg(feature = "System+Xml+Schema+FacetsChecker")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::FacetsChecker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FacetsChecker_FacetsCompiler {
    pub datatype: *mut crate::System::Xml::Schema::DatatypeImplementation,
    pub derivedRestriction: *mut crate::System::Xml::Schema::RestrictionFacets,
    pub baseFlags: crate::System::Xml::Schema::RestrictionFlags,
    pub baseFixedFlags: crate::System::Xml::Schema::RestrictionFlags,
    pub validRestrictionFlags: crate::System::Xml::Schema::RestrictionFlags,
    pub nonNegativeInt: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    pub builtInType: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    pub builtInEnum: crate::System::Xml::Schema::XmlTypeCode,
    pub firstPattern: bool,
    pub regStr: *mut crate::System::Text::StringBuilder,
    pub pattern_facet: *mut crate::System::Xml::Schema::XmlSchemaPatternFacet,
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::FacetsChecker_FacetsCompiler => "System.Xml.Schema"
    ."FacetsChecker/FacetsCompiler"
);
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::FacetsChecker_FacetsCompiler {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler")]
impl crate::System::Xml::Schema::FacetsChecker_FacetsCompiler {
    #[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler+Map")]
    pub type Map = crate::System::Xml::Schema::FacetsCompiler_Map;
    pub fn CheckDupFlag(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
        flag: crate::System::Xml::Schema::RestrictionFlags,
        errorCode: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckDupFlag",
            (facet, flag, errorCode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CheckProhibitedFlag(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
        flag: crate::System::Xml::Schema::RestrictionFlags,
        errorCode: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckProhibitedFlag",
            (facet, flag, errorCode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CheckValue(
        &mut self,
        value: *mut crate::System::Object,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckValue",
            (value, facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileEnumerationFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
        nsmgr: *mut crate::System::Xml::IXmlNamespaceResolver,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileEnumerationFacet",
            (facet, nsmgr, nameTable),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileFacetCombinations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileFacetCombinations",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileFractionDigitsFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileFractionDigitsFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileLengthFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileLengthFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMaxExclusiveFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMaxExclusiveFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMaxInclusiveFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMaxInclusiveFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMaxLengthFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMaxLengthFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMinExclusiveFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMinExclusiveFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMinInclusiveFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMinInclusiveFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileMinLengthFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileMinLengthFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompilePatternFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaPatternFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompilePatternFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileTotalDigitsFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileTotalDigitsFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompileWhitespaceFacet(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompileWhitespaceFacet",
            (facet),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyFacetsFromBaseType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFacetsFromBaseType",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FinishFacetCompile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinishFacetCompile",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ParseFacetValue(
        &mut self,
        datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
        code: *mut crate::System::String,
        nsmgr: *mut crate::System::Xml::IXmlNamespaceResolver,
        nameTable: *mut crate::System::Xml::XmlNameTable,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseFacetValue",
            (datatype, facet, code, nsmgr, nameTable),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFlag_RestrictionFlags1(
        &mut self,
        flag: crate::System::Xml::Schema::RestrictionFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFlag",
            (flag),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFlag_XmlSchemaFacet_RestrictionFlags0(
        &mut self,
        facet: *mut crate::System::Xml::Schema::XmlSchemaFacet,
        flag: crate::System::Xml::Schema::RestrictionFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFlag",
            (facet, flag),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        baseDatatype: *mut crate::System::Xml::Schema::DatatypeImplementation,
        restriction: *mut crate::System::Xml::Schema::RestrictionFacets,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (baseDatatype, restriction),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler+Map")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FacetsCompiler_Map {
    pub _cordl_match: char,
    pub replacement: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler+Map")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::FacetsCompiler_Map =>
    "System.Xml.Schema"."FacetsChecker/FacetsCompiler/Map"
);
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler+Map")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::FacetsCompiler_Map {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+FacetsChecker+FacetsCompiler+Map")]
impl crate::System::Xml::Schema::FacetsCompiler_Map {
    pub fn _ctor(
        &mut self,
        m: char,
        r: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m, r),
        )?;
        Ok(__cordl_ret)
    }
}

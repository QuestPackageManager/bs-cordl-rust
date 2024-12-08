#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct DfaContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub transitionTable: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
}
#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::DfaContentValidator =>
    "System.Xml.Schema"."DfaContentValidator"
);
#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::DfaContentValidator {
    type Target = crate::System::Xml::Schema::ContentValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::DfaContentValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
impl crate::System::Xml::Schema::DfaContentValidator {
    pub fn ValidateElement(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        context: *mut crate::System::Xml::Schema::ValidationState,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ValidateElement", (name, context, errorCode))?;
        Ok(__cordl_ret)
    }
    pub fn InitValidation(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitValidation", (context))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteValidation(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompleteValidation", (context))?;
        Ok(__cordl_ret)
    }
    pub fn ExpectedElements(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
        isRequiredOnly: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("ExpectedElements", (context, isRequiredOnly))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        transitionTable: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isOpen: bool,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (transitionTable, symbols, contentType, isOpen, isEmptiable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ExpectedParticles(
        &mut self,
        context: *mut crate::System::Xml::Schema::ValidationState,
        isRequiredOnly: bool,
        schemaSet: *mut crate::System::Xml::Schema::XmlSchemaSet,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("ExpectedParticles", (context, isRequiredOnly, schemaSet))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        transitionTable: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        symbols: *mut crate::System::Xml::Schema::SymbolsDictionary,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isOpen: bool,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (transitionTable, symbols, contentType, isOpen, isEmptiable),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+DfaContentValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::DfaContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct AllElementsContentValidator {
    __cordl_parent: crate::System::Xml::Schema::ContentValidator,
    pub elements: *mut crate::System::Collections::Hashtable,
    pub particles: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub isRequired: *mut crate::System::Xml::Schema::BitSet,
    pub countRequired: i32,
}
#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::AllElementsContentValidator
    => "System.Xml.Schema"."AllElementsContentValidator"
);
#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::AllElementsContentValidator {
    type Target = crate::System::Xml::Schema::ContentValidator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::AllElementsContentValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
impl crate::System::Xml::Schema::AllElementsContentValidator {
    pub fn AddElement(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        particle: *mut quest_hook::libil2cpp::Il2CppObject,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddElement", (name, particle, isEmptiable))?;
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
    pub fn New(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        _cordl_size: i32,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType, _cordl_size, isEmptiable))?;
        Ok(__cordl_object)
    }
    pub fn ValidateElement(
        &mut self,
        name: *mut crate::System::Xml::XmlQualifiedName,
        context: *mut crate::System::Xml::Schema::ValidationState,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ValidateElement", (name, context, errorCode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        _cordl_size: i32,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType, _cordl_size, isEmptiable))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsEmptiable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptiable", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+AllElementsContentValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::AllElementsContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Schema+ContentValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ContentValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    pub isOpen: bool,
    pub isEmptiable: bool,
}
#[cfg(feature = "System+Xml+Schema+ContentValidator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::ContentValidator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "ContentValidator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+Schema+ContentValidator")]
impl std::ops::Deref for crate::System::Xml::Schema::ContentValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ContentValidator")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ContentValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ContentValidator")]
impl crate::System::Xml::Schema::ContentValidator {
    pub fn AddParticleToExpected_XmlSchemaParticle_XmlSchemaSet_ArrayList0(
        p: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
        particles: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddParticleToExpected", (p, schemaSet, particles))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddParticleToExpected__cordl_bool1(
        p: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
        particles: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        global: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddParticleToExpected", (p, schemaSet, particles, global))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CompleteValidation", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectedElements(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("ExpectedElements", (context, isRequiredOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectedParticles(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        isRequiredOnly: bool,
        schemaSet: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object
            .invoke("ExpectedParticles", (context, isRequiredOnly, schemaSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitValidation(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitValidation", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_XmlSchemaContentType0(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool__cordl_bool1(
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isOpen: bool,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contentType, isOpen, isEmptiable))?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateElement(
        &mut self,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        context: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationState>,
        errorCode: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ValidateElement", (name, context, errorCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaContentType0(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool__cordl_bool1(
        &mut self,
        contentType: crate::System::Xml::Schema::XmlSchemaContentType,
        isOpen: bool,
        isEmptiable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (contentType, isOpen, isEmptiable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = __cordl_object
            .invoke("get_ContentType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmptiable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsEmptiable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsOpen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsOpen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PreserveWhitespace(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_PreserveWhitespace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsOpen(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsOpen", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ContentValidator")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ContentValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

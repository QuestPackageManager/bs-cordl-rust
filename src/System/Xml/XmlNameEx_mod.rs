#[cfg(feature = "System+Xml+XmlNameEx")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNameEx {
    __cordl_parent: crate::System::Xml::XmlName,
    pub flags: u8,
    pub memberType: *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    pub schemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
    pub decl: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+XmlNameEx")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlNameEx => "System.Xml"
    ."XmlNameEx"
);
#[cfg(feature = "System+Xml+XmlNameEx")]
impl std::ops::Deref for crate::System::Xml::XmlNameEx {
    type Target = crate::System::Xml::XmlName;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNameEx")]
impl std::ops::DerefMut for crate::System::Xml::XmlNameEx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNameEx")]
impl crate::System::Xml::XmlNameEx {
    pub fn Equals(
        &mut self,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (schemaInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        hashCode: i32,
        ownerDoc: *mut crate::System::Xml::XmlDocument,
        next: *mut crate::System::Xml::XmlName,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (prefix, localName, ns, hashCode, ownerDoc, next, schemaInfo),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetIsDefault(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsDefault", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsNil(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsNil", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetValidity(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaValidity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValidity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        localName: *mut crate::System::String,
        ns: *mut crate::System::String,
        hashCode: i32,
        ownerDoc: *mut crate::System::Xml::XmlDocument,
        next: *mut crate::System::Xml::XmlName,
        schemaInfo: *mut crate::System::Xml::Schema::IXmlSchemaInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (prefix, localName, ns, hashCode, ownerDoc, next, schemaInfo),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNil(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNil", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaSimpleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaSimpleType = __cordl_object
            .invoke("get_MemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAttribute = __cordl_object
            .invoke("get_SchemaAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaElement = __cordl_object
            .invoke("get_SchemaElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaType = __cordl_object
            .invoke("get_SchemaType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Validity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaValidity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidity = __cordl_object
            .invoke("get_Validity", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XmlNameEx")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlNameEx {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

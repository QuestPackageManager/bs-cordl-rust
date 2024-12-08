#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaAnnotated {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaObject,
    pub id: *mut crate::System::String,
    pub annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    pub moreAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::XmlAttribute,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaAnnotated =>
    "System.Xml.Schema"."XmlSchemaAnnotated"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaAnnotated {
    type Target = crate::System::Xml::Schema::XmlSchemaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaAnnotated {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
impl crate::System::Xml::Schema::XmlSchemaAnnotated {
    pub fn get_Annotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaAnnotation = __cordl_object
            .invoke("get_Annotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UnhandledAttributes(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UnhandledAttributes", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_UnhandledAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        > = __cordl_object.invoke("get_UnhandledAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_IdAttribute(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IdAttribute", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddAnnotation(
        &mut self,
        annotation: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAnnotation", (annotation))?;
        Ok(__cordl_ret)
    }
    pub fn SetUnhandledAttributes(
        &mut self,
        moreAttributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::XmlAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUnhandledAttributes", (moreAttributes))?;
        Ok(__cordl_ret)
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
    pub fn set_Id(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Id", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IdAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_IdAttribute", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Annotation(
        &mut self,
        value: *mut crate::System::Xml::Schema::XmlSchemaAnnotation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Annotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Id", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnnotated")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaAnnotated {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

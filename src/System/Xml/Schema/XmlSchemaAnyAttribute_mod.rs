#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaAnyAttribute {
    __cordl_parent: crate::System::Xml::Schema::XmlSchemaAnnotated,
    pub ns: *mut quest_hook::libil2cpp::Il2CppString,
    pub processContents: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    pub namespaceList: *mut crate::System::Xml::Schema::NamespaceList,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaAnyAttribute =>
    "System.Xml.Schema"."XmlSchemaAnyAttribute"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaAnyAttribute {
    type Target = crate::System::Xml::Schema::XmlSchemaAnnotated;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaAnyAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
impl crate::System::Xml::Schema::XmlSchemaAnyAttribute {
    pub fn Allows(
        &mut self,
        qname: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Allows", (qname))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNamespaceList(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildNamespaceList", (targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildNamespaceListV1Compat(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildNamespaceListV1Compat", (targetNamespace))?;
        Ok(__cordl_ret.into())
    }
    pub fn Intersection(
        o1: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        o2: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        v1Compat: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Intersection", (o1, o2, v1Compat))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSubset(
        sub: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
        _cordl_super: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSubset", (sub, _cordl_super))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Union(
        o1: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        o2: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        v1Compat: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Union", (o1, o2, v1Compat))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamespaceList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::NamespaceList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::NamespaceList,
        > = __cordl_object.invoke("get_NamespaceList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessContents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentProcessing,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentProcessing = __cordl_object
            .invoke("get_ProcessContents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProcessContentsCorrect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentProcessing,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentProcessing = __cordl_object
            .invoke("get_ProcessContentsCorrect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Namespace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Namespace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ProcessContents(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaContentProcessing,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProcessContents", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaAnyAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaAnyAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

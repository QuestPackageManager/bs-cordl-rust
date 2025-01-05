#[cfg(feature = "System+Xml+DomNameTable")]
#[repr(C)]
#[derive(Debug)]
pub struct DomNameTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub entries: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlName>,
    >,
    pub count: i32,
    pub mask: i32,
    pub ownerDocument: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    pub nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
}
#[cfg(feature = "System+Xml+DomNameTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::DomNameTable => "System.Xml"
    ."DomNameTable"
);
#[cfg(feature = "System+Xml+DomNameTable")]
impl std::ops::Deref for crate::System::Xml::DomNameTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DomNameTable")]
impl std::ops::DerefMut for crate::System::Xml::DomNameTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+DomNameTable")]
impl crate::System::Xml::DomNameTable {
    pub fn AddName(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schemaInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IXmlSchemaInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlName> = __cordl_object
            .invoke("AddName", (prefix, localName, ns, schemaInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schemaInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IXmlSchemaInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlName>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlName> = __cordl_object
            .invoke("GetName", (prefix, localName, ns, schemaInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Grow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Grow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        document: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (document))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        document: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDocument>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (document))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+DomNameTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::DomNameTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

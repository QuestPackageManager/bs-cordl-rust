#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlListConverter {
    __cordl_parent: crate::System::Xml::Schema::XmlBaseConverter,
    pub atomicConverter: *mut crate::System::Xml::Schema::XmlValueConverter,
}
#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlListConverter =>
    "System.Xml.Schema"."XmlListConverter"
);
#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlListConverter {
    type Target = crate::System::Xml::Schema::XmlBaseConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlListConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
impl crate::System::Xml::Schema::XmlListConverter {
    pub fn ChangeListType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ChangeListType", (value, destinationType, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ChangeType", (value, destinationType, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInvalidClrMappingException(
        &mut self,
        sourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = __cordl_object
            .invoke("CreateInvalidClrMappingException", (sourceType, destinationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsListType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsListType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ListAsString(
        &mut self,
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ListAsString", (list, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_XmlBaseConverter0(
        atomicConverter: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlBaseConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atomicConverter))?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlBaseConverter_Type1(
        atomicConverter: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlBaseConverter,
        >,
        clrTypeDefault: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atomicConverter, clrTypeDefault))?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSchemaType2(
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType))?;
        Ok(__cordl_object.into())
    }
    pub fn StringAsList(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("StringAsList", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArray<T>(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("ToArray", (list, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToList(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("ToList", (list, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlBaseConverter0(
        &mut self,
        atomicConverter: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlBaseConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atomicConverter))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlBaseConverter_Type1(
        &mut self,
        atomicConverter: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlBaseConverter,
        >,
        clrTypeDefault: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atomicConverter, clrTypeDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaType2(
        &mut self,
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schemaType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlListConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlListConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

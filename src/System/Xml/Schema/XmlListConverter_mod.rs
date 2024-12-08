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
    pub fn _ctor_XmlBaseConverter0(
        &mut self,
        atomicConverter: *mut crate::System::Xml::Schema::XmlBaseConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atomicConverter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlBaseConverter_Type1(
        &mut self,
        atomicConverter: *mut crate::System::Xml::Schema::XmlBaseConverter,
        clrTypeDefault: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (atomicConverter, clrTypeDefault))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlSchemaType2(
        &mut self,
        schemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (schemaType))?;
        Ok(__cordl_ret)
    }
    pub fn IsListType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsListType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInvalidClrMappingException(
        &mut self,
        sourceType: *mut crate::System::Type,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("CreateInvalidClrMappingException", (sourceType, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ToArray<T>(
        &mut self,
        list: *mut crate::System::Object,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = __cordl_object
            .invoke("ToArray", (list, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeListType(
        &mut self,
        value: *mut crate::System::Object,
        destinationType: *mut crate::System::Type,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeListType", (value, destinationType, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn ToList(
        &mut self,
        list: *mut crate::System::Object,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("ToList", (list, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn StringAsList(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("StringAsList", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ListAsString(
        &mut self,
        list: *mut crate::System::Collections::IEnumerable,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ListAsString", (list, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType(
        &mut self,
        value: *mut crate::System::Object,
        destinationType: *mut crate::System::Type,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn New_XmlBaseConverter0(
        atomicConverter: *mut crate::System::Xml::Schema::XmlBaseConverter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atomicConverter))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlBaseConverter_Type1(
        atomicConverter: *mut crate::System::Xml::Schema::XmlBaseConverter,
        clrTypeDefault: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (atomicConverter, clrTypeDefault))?;
        Ok(__cordl_object)
    }
    pub fn New_XmlSchemaType2(
        schemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType))?;
        Ok(__cordl_object)
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

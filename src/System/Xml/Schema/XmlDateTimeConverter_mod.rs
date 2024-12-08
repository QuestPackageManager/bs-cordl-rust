#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlDateTimeConverter {
    __cordl_parent: crate::System::Xml::Schema::XmlBaseConverter,
}
#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlDateTimeConverter =>
    "System.Xml.Schema"."XmlDateTimeConverter"
);
#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlDateTimeConverter {
    type Target = crate::System::Xml::Schema::XmlBaseConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlDateTimeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
impl crate::System::Xml::Schema::XmlDateTimeConverter {
    pub fn ToDateTimeOffset_DateTime0(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTimeOffset = __cordl_object
            .invoke("ToDateTimeOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTimeOffset_String1(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTimeOffset = __cordl_object
            .invoke("ToDateTimeOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTimeOffset_Object2(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTimeOffset = __cordl_object
            .invoke("ToDateTimeOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime_DateTimeOffset0(
        &mut self,
        value: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime_String1(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime_Object2(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString_DateTime0(
        &mut self,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString_DateTimeOffset1(
        &mut self,
        value: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToString_Object_IXmlNamespaceResolver2(
        &mut self,
        value: *mut crate::System::Object,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (value, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_DateTime0(
        &mut self,
        value: crate::System::DateTime,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_String_IXmlNamespaceResolver1(
        &mut self,
        value: *mut crate::System::String,
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
    pub fn ChangeType_Object_IXmlNamespaceResolver2(
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
    pub fn _ctor(
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
    pub fn New(
        schemaType: *mut crate::System::Xml::Schema::XmlSchemaType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlDateTimeConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlDateTimeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

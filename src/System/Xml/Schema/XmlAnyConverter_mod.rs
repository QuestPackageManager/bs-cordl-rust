#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAnyConverter {
    __cordl_parent: crate::System::Xml::Schema::XmlBaseConverter,
}
#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlAnyConverter =>
    "System.Xml.Schema"."XmlAnyConverter"
);
#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlAnyConverter {
    type Target = crate::System::Xml::Schema::XmlBaseConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlAnyConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
impl crate::System::Xml::Schema::XmlAnyConverter {
    pub fn ChangeTypeWildcardDestination(
        &mut self,
        value: *mut crate::System::Object,
        destinationType: *mut crate::System::Type,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "ChangeTypeWildcardDestination",
                (value, destinationType, nsResolver),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ChangeTypeWildcardSource(
        &mut self,
        value: *mut crate::System::Object,
        destinationType: *mut crate::System::Type,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeTypeWildcardSource", (value, destinationType, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_DateTime1(
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
    pub fn ChangeType_Decimal2(
        &mut self,
        value: crate::System::Decimal,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_Object_IXmlNamespaceResolver7(
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
    pub fn ChangeType_String_IXmlNamespaceResolver6(
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
    pub fn ChangeType__cordl_bool0(
        &mut self,
        value: bool,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_f64_3(
        &mut self,
        value: f64,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_i32_4(
        &mut self,
        value: i32,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ChangeType_i64_5(
        &mut self,
        value: i64,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ChangeType", (value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        typeCode: crate::System::Xml::Schema::XmlTypeCode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeCode))?;
        Ok(__cordl_object)
    }
    pub fn ToBoolean(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBoolean", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime(
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
    pub fn ToDateTimeOffset(
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
    pub fn ToDecimal(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ToDecimal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToDouble(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ToDouble", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt32(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToInt32", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt64(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ToInt64", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ToNavigator(
        &mut self,
        nav: *mut crate::System::Xml::XPath::XPathNavigator,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::XPath::XPathNavigator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::XPath::XPathNavigator = __cordl_object
            .invoke("ToNavigator", (nav))?;
        Ok(__cordl_ret)
    }
    pub fn ToSingle(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ToSingle", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        typeCode: crate::System::Xml::Schema::XmlTypeCode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeCode))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAnyConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlAnyConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAtomicValue_NamespacePrefixForQName {
    __cordl_parent: crate::System::Object,
    pub prefix: *mut crate::System::String,
    pub ns: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName => "System.Xml.Schema"
    ."XmlAtomicValue/NamespacePrefixForQName"
);
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl std::ops::Deref
for crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    pub fn GetNamespacesInScope(
        &mut self,
        scope: crate::System::Xml::XmlNamespaceScope,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret)
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret)
    }
    pub fn LookupPrefix(
        &mut self,
        namespaceName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        prefix: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefix, ns))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        prefix: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefix, ns))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlAtomicValue_Union {
    padding: [u8; 8usize],
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlAtomicValue_Union =>
    "System.Xml.Schema"."XmlAtomicValue/Union"
);
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::XmlAtomicValue_Union {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
impl crate::System::Xml::Schema::XmlAtomicValue_Union {}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAtomicValue {
    __cordl_parent: crate::System::Xml::XPath::XPathItem,
    pub xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
    pub objVal: *mut crate::System::Object,
    pub clrType: crate::System::TypeCode,
    pub unionVal: crate::System::Xml::Schema::XmlAtomicValue_Union,
    pub nsPrefix: *mut crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName,
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlAtomicValue =>
    "System.Xml.Schema"."XmlAtomicValue"
);
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlAtomicValue {
    type Target = crate::System::Xml::XPath::XPathItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlAtomicValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl crate::System::Xml::Schema::XmlAtomicValue {
    #[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
    pub type NamespacePrefixForQName = crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName;
    #[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
    pub type Union = crate::System::Xml::Schema::XmlAtomicValue_Union;
    pub fn GetPrefixFromQName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPrefixFromQName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_DateTime1(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_Object7(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_Object_IXmlNamespaceResolver8(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::Object,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_object)
    }
    pub fn New_String5(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_String_IXmlNamespaceResolver6(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::String,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool0(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_f64_2(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_3(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn New_i64_4(
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object)
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValueAs(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ValueAs", (_cordl_type, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTime1(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object7(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object_IXmlNamespaceResolver8(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::Object,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String5(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_IXmlNamespaceResolver6(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: *mut crate::System::String,
        nsResolver: *mut crate::System::Xml::IXmlNamespaceResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f64_2(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_3(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_4(
        &mut self,
        xmlType: *mut crate::System::Xml::Schema::XmlSchemaType,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_TypedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_TypedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueAsBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ValueAsBoolean", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueAsDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_ValueAsDateTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueAsDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_ValueAsDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueAsInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueAsInt", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueAsLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ValueAsLong", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::XmlSchemaType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaType = __cordl_object
            .invoke("get_XmlType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlAtomicValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

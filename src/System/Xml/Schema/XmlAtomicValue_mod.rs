#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAtomicValue {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Xml::XPath::XPathItem>,
    pub xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    pub objVal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub clrType: crate::System::TypeCode,
    pub unionVal: crate::System::Xml::Schema::XmlAtomicValue_Union,
    pub nsPrefix: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlAtomicValue =>
    "System.Xml.Schema"."XmlAtomicValue"
);
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlAtomicValue {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::XPath::XPathItem>;
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
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPrefixFromQName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DateTime1(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc5(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc7(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_Gc6(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_Gc8(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool0(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_f64_2(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_3(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i64_4(
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlType, value))?;
        Ok(__cordl_object.into())
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueAs(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ValueAs", (_cordl_type, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime1(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc5(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc7(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc6(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc8(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_2(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_3(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_4(
        &mut self,
        xmlType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlType, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypedValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_TypedValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueAsBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ValueAsBoolean", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueAsDateTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_ValueAsDateTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueAsDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_ValueAsDouble", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueAsInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ValueAsInt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueAsLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ValueAsLong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ValueType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = __cordl_object.invoke("get_XmlType", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::ICloneable>>
for crate::System::Xml::Schema::XmlAtomicValue {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::ICloneable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::ICloneable>>
for crate::System::Xml::Schema::XmlAtomicValue {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::ICloneable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAtomicValue_NamespacePrefixForQName {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetNamespacesInScope", (scope))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupNamespace(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupNamespace", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupPrefix(
        &mut self,
        namespaceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LookupPrefix", (namespaceName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prefix, ns))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prefix, ns))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>>
for crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+NamespacePrefixForQName")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>>
for crate::System::Xml::Schema::XmlAtomicValue_NamespacePrefixForQName {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAtomicValue+Union")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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

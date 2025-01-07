#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlStringConverter {
    __cordl_parent: crate::System::Xml::Schema::XmlBaseConverter,
}
#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XmlStringConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XmlStringConverter";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlStringConverter {
    type Target = crate::System::Xml::Schema::XmlBaseConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlStringConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
impl crate::System::Xml::Schema::XmlStringConverter {
    pub fn ChangeType_Il2CppObject1(
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
    pub fn ChangeType_Il2CppString0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub fn Create(
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlValueConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlValueConverter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (schemaType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (schemaType))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nsResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlNamespaceResolver>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (value, nsResolver))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
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
#[cfg(feature = "System+Xml+Schema+XmlStringConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlStringConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAnyListConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlListConverter,
    >,
}
#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlAnyListConverter =>
    "System.Xml.Schema"."XmlAnyListConverter"
);
#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlAnyListConverter {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlListConverter,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlAnyListConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
impl crate::System::Xml::Schema::XmlAnyListConverter {
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
    pub fn New(
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
    pub fn _ctor(
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
}
#[cfg(feature = "System+Xml+Schema+XmlAnyListConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlAnyListConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAttributes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub xmlAnyAttribute: *mut crate::System::Xml::Serialization::XmlAnyAttributeAttribute,
    pub xmlAnyElements: *mut crate::System::Xml::Serialization::XmlAnyElementAttributes,
    pub xmlArray: *mut crate::System::Xml::Serialization::XmlArrayAttribute,
    pub xmlArrayItems: *mut crate::System::Xml::Serialization::XmlArrayItemAttributes,
    pub xmlAttribute: *mut crate::System::Xml::Serialization::XmlAttributeAttribute,
    pub xmlChoiceIdentifier: *mut crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute,
    pub xmlDefaultValue: *mut quest_hook::libil2cpp::Il2CppObject,
    pub xmlElements: *mut crate::System::Xml::Serialization::XmlElementAttributes,
    pub xmlEnum: *mut crate::System::Xml::Serialization::XmlEnumAttribute,
    pub xmlIgnore: bool,
    pub xmlns: bool,
    pub xmlRoot: *mut crate::System::Xml::Serialization::XmlRootAttribute,
    pub xmlText: *mut crate::System::Xml::Serialization::XmlTextAttribute,
    pub xmlType: *mut crate::System::Xml::Serialization::XmlTypeAttribute,
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlAttributes =>
    "System.Xml.Serialization"."XmlAttributes"
);
#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlAttributes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
impl crate::System::Xml::Serialization::XmlAttributes {
    pub fn AddKeyHash(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyHash", (sb))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ICustomAttributeProvider1(
        provider: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ICustomAttributeProvider1(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ICustomAttributeProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Order(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_Order", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SortableOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SortableOrder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlAnyAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAnyAttributeAttribute,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAnyAttributeAttribute,
        > = __cordl_object.invoke("get_XmlAnyAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlAnyElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAnyElementAttributes,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAnyElementAttributes,
        > = __cordl_object.invoke("get_XmlAnyElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlArrayAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlArrayAttribute,
        > = __cordl_object.invoke("get_XmlArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlArrayItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlArrayItemAttributes,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlArrayItemAttributes,
        > = __cordl_object.invoke("get_XmlArrayItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeAttribute,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeAttribute,
        > = __cordl_object.invoke("get_XmlAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlChoiceIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlChoiceIdentifierAttribute,
        > = __cordl_object.invoke("get_XmlChoiceIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlDefaultValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_XmlDefaultValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlElementAttributes,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlElementAttributes,
        > = __cordl_object.invoke("get_XmlElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlIgnore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_XmlIgnore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlRootAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        > = __cordl_object.invoke("get_XmlRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlTextAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTextAttribute,
        > = __cordl_object.invoke("get_XmlText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlTypeAttribute>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeAttribute,
        > = __cordl_object.invoke("get_XmlType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Xmlns(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Xmlns", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributes")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlAttributes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

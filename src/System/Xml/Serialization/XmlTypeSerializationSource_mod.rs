#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeSerializationSource {
    __cordl_parent: crate::System::Xml::Serialization::SerializationSource,
    pub attributeOverridesHash: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub rootHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Serialization::XmlTypeSerializationSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Serialization";
    const CLASS_NAME: &'static str = "XmlTypeSerializationSource";
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
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeSerializationSource {
    type Target = crate::System::Xml::Serialization::SerializationSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlTypeSerializationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
impl crate::System::Xml::Serialization::XmlTypeSerializationSource {
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        attributeOverrides: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeOverrides,
        >,
        namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includedTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, root, attributeOverrides, namspace, includedTypes),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
        attributeOverrides: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlAttributeOverrides,
        >,
        namspace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includedTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, root, attributeOverrides, namspace, includedTypes),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeSerializationSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

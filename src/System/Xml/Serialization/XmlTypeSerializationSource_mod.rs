#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeSerializationSource {
    __cordl_parent: crate::System::Xml::Serialization::SerializationSource,
    pub attributeOverridesHash: *mut crate::System::String,
    pub _cordl_type: *mut crate::System::Type,
    pub rootHash: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeSerializationSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeSerializationSource =>
    "System.Xml.Serialization"."XmlTypeSerializationSource"
);
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
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        attributeOverrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        namspace: *mut crate::System::String,
        includedTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, root, attributeOverrides, namspace, includedTypes),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        attributeOverrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        namspace: *mut crate::System::String,
        includedTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, root, attributeOverrides, namspace, includedTypes),
            )?;
        Ok(__cordl_ret)
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

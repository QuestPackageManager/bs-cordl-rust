#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub includedTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub namspace: *mut quest_hook::libil2cpp::Il2CppString,
    pub canBeGenerated: bool,
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::SerializationSource
    => "System.Xml.Serialization"."SerializationSource"
);
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl std::ops::Deref for crate::System::Xml::Serialization::SerializationSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::SerializationSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl crate::System::Xml::Serialization::SerializationSource {
    pub fn BaseEquals(
        &mut self,
        other: *mut crate::System::Xml::Serialization::SerializationSource,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("BaseEquals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        namspace: *mut quest_hook::libil2cpp::Il2CppString,
        includedTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namspace, includedTypes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        namspace: *mut quest_hook::libil2cpp::Il2CppString,
        includedTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namspace, includedTypes))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+SerializationSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::SerializationSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

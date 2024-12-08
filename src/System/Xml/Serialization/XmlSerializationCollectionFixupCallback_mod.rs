#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializationCollectionFixupCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlSerializationCollectionFixupCallback =>
    "System.Xml.Serialization"."XmlSerializationCollectionFixupCallback"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
impl std::ops::Deref
for crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
impl std::ops::DerefMut
for crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
impl crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        collection: *mut crate::System::Object,
        collectionItems: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (collection, collectionItems))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializationCollectionFixupCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializationCollectionFixupCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

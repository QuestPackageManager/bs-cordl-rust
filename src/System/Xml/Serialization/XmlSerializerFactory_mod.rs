#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSerializerFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlSerializerFactory
    => "System.Xml.Serialization"."XmlSerializerFactory"
);
#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlSerializerFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlSerializerFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
impl crate::System::Xml::Serialization::XmlSerializerFactory {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSerializer_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializer = __cordl_object
            .invoke("CreateSerializer", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSerializer_XmlRootAttribute1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializer = __cordl_object
            .invoke("CreateSerializer", (_cordl_type, root))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSerializer_XmlAttributeOverrides_Il2CppArray_XmlRootAttribute_String2(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        overrides: *mut crate::System::Xml::Serialization::XmlAttributeOverrides,
        extraTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        root: *mut crate::System::Xml::Serialization::XmlRootAttribute,
        defaultNamespace: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlSerializer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlSerializer = __cordl_object
            .invoke(
                "CreateSerializer",
                (_cordl_type, overrides, extraTypes, root, defaultNamespace),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlSerializerFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlSerializerFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

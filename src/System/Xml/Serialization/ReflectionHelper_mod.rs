#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionHelper {
    __cordl_parent: crate::System::Object,
    pub _clrTypes: *mut crate::System::Collections::Hashtable,
    pub _schemaTypes: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::ReflectionHelper =>
    "System.Xml.Serialization"."ReflectionHelper"
);
#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
impl std::ops::Deref for crate::System::Xml::Serialization::ReflectionHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::ReflectionHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
impl crate::System::Xml::Serialization::ReflectionHelper {
    pub fn GetRegisteredClrType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("GetRegisteredClrType", (_cordl_type, ns))?;
        Ok(__cordl_ret)
    }
    pub fn GetRegisteredSchemaType(
        &mut self,
        xmlType: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapping,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapping = __cordl_object
            .invoke("GetRegisteredSchemaType", (xmlType, ns))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RegisterClrType(
        &mut self,
        map: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        _cordl_type: *mut crate::System::Type,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterClrType", (map, _cordl_type, ns))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterSchemaType(
        &mut self,
        map: *mut crate::System::Xml::Serialization::XmlTypeMapping,
        xmlType: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterSchemaType", (map, xmlType, ns))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Xml+Serialization+ReflectionHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::ReflectionHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

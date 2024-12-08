#[cfg(feature = "System+Xml+Serialization+EnumMap")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumMap {
    __cordl_parent: crate::System::Xml::Serialization::ObjectMap,
    pub _members: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Serialization::EnumMap_EnumMapMember,
    >,
    pub _isFlags: bool,
    pub _enumNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _xmlNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub _values: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
}
#[cfg(feature = "System+Xml+Serialization+EnumMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::EnumMap =>
    "System.Xml.Serialization"."EnumMap"
);
#[cfg(feature = "System+Xml+Serialization+EnumMap")]
impl std::ops::Deref for crate::System::Xml::Serialization::EnumMap {
    type Target = crate::System::Xml::Serialization::ObjectMap;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::EnumMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap")]
impl crate::System::Xml::Serialization::EnumMap {
    #[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
    pub type EnumMapMember = crate::System::Xml::Serialization::EnumMap_EnumMapMember;
    pub fn GetEnumName(
        &mut self,
        typeName: *mut crate::System::String,
        xmlName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEnumName", (typeName, xmlName))?;
        Ok(__cordl_ret)
    }
    pub fn GetXmlName(
        &mut self,
        typeName: *mut crate::System::String,
        enumValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetXmlName", (typeName, enumValue))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        members: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Serialization::EnumMap_EnumMapMember,
        >,
        isFlags: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (members, isFlags))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        members: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Serialization::EnumMap_EnumMapMember,
        >,
        isFlags: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (members, isFlags))?;
        Ok(__cordl_ret)
    }
    pub fn get_EnumNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_EnumNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFlags(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFlags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i64>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i64> = __cordl_object
            .invoke("get_Values", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_XmlNames", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::EnumMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumMap_EnumMapMember {
    __cordl_parent: crate::System::Object,
    pub _xmlName: *mut crate::System::String,
    pub _enumName: *mut crate::System::String,
    pub _value: i64,
}
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::EnumMap_EnumMapMember => "System.Xml.Serialization"
    ."EnumMap/EnumMapMember"
);
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
impl std::ops::Deref for crate::System::Xml::Serialization::EnumMap_EnumMapMember {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::EnumMap_EnumMapMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
impl crate::System::Xml::Serialization::EnumMap_EnumMapMember {
    pub fn New(
        xmlName: *mut crate::System::String,
        enumName: *mut crate::System::String,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (xmlName, enumName, value))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        xmlName: *mut crate::System::String,
        enumName: *mut crate::System::String,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (xmlName, enumName, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_EnumName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EnumName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+EnumMap+EnumMapMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::EnumMap_EnumMapMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

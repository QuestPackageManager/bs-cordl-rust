#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlReflectionMember {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub isReturnValue: bool,
    pub memberName: *mut quest_hook::libil2cpp::Il2CppString,
    pub memberType: *mut crate::System::Type,
    pub xmlAttributes: *mut crate::System::Xml::Serialization::XmlAttributes,
    pub declaringType: *mut crate::System::Type,
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::XmlReflectionMember
    => "System.Xml.Serialization"."XmlReflectionMember"
);
#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlReflectionMember {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlReflectionMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
impl crate::System::Xml::Serialization::XmlReflectionMember {
    pub fn New(
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        attributes: *mut crate::System::Xml::Serialization::XmlAttributes,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, _cordl_type, attributes))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        _cordl_type: *mut crate::System::Type,
        attributes: *mut crate::System::Xml::Serialization::XmlAttributes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, _cordl_type, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn get_DeclaringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_DeclaringType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReturnValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReturnValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_MemberName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_MemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlAttributes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlAttributes = __cordl_object
            .invoke("get_XmlAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_DeclaringType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DeclaringType", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlReflectionMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlReflectionMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

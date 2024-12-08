#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAttributeOverrides {
    __cordl_parent: crate::System::Object,
    pub overrides: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlAttributeOverrides => "System.Xml.Serialization"
    ."XmlAttributeOverrides"
);
#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlAttributeOverrides {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlAttributeOverrides {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
impl crate::System::Xml::Serialization::XmlAttributeOverrides {
    pub fn GetKey(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        member: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::TypeMember,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::TypeMember = __cordl_object
            .invoke("GetKey", (_cordl_type, member))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_Type0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlAttributes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlAttributes = __cordl_object
            .invoke("get_Item", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        member: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlAttributes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlAttributes = __cordl_object
            .invoke("get_Item", (_cordl_type, member))?;
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
    pub fn AddKeyHash(
        &mut self,
        sb: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddKeyHash", (sb))?;
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
#[cfg(feature = "System+Xml+Serialization+XmlAttributeOverrides")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlAttributeOverrides {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlTypeMapMemberElement {
    __cordl_parent: crate::System::Xml::Serialization::XmlTypeMapMember,
    pub _elementInfo: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    pub _choiceMember: *mut crate::System::String,
    pub _isTextCollector: bool,
    pub _choiceTypeData: *mut crate::System::Xml::Serialization::TypeData,
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Serialization::XmlTypeMapMemberElement => "System.Xml.Serialization"
    ."XmlTypeMapMemberElement"
);
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
impl std::ops::Deref for crate::System::Xml::Serialization::XmlTypeMapMemberElement {
    type Target = crate::System::Xml::Serialization::XmlTypeMapMember;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::XmlTypeMapMemberElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
impl crate::System::Xml::Serialization::XmlTypeMapMemberElement {
    pub fn set_ElementInfo(
        &mut self,
        value: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ElementInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChoiceTypeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::TypeData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::TypeData = __cordl_object
            .invoke("get_ChoiceTypeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindElement(
        &mut self,
        ob: *mut crate::System::Object,
        memberValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("FindElement", (ob, memberValue))?;
        Ok(__cordl_ret)
    }
    pub fn set_ChoiceTypeData(
        &mut self,
        value: *mut crate::System::Xml::Serialization::TypeData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChoiceTypeData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsXmlTextCollector(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsXmlTextCollector", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChoiceMember(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ChoiceMember", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsXmlTextCollector(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsXmlTextCollector", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ElementInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList = __cordl_object
            .invoke("get_ElementInfo", ())?;
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
    pub fn set_ChoiceMember(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChoiceMember", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetChoice(
        &mut self,
        ob: *mut crate::System::Object,
        choice: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetChoice", (ob, choice))?;
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
#[cfg(feature = "System+Xml+Serialization+XmlTypeMapMemberElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Serialization::XmlTypeMapMemberElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

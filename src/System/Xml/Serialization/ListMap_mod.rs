#[cfg(feature = "System+Xml+Serialization+ListMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ListMap {
    __cordl_parent: crate::System::Xml::Serialization::ObjectMap,
    pub _itemInfo: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    pub _choiceMember: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Serialization+ListMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Serialization::ListMap =>
    "System.Xml.Serialization"."ListMap"
);
#[cfg(feature = "System+Xml+Serialization+ListMap")]
impl std::ops::Deref for crate::System::Xml::Serialization::ListMap {
    type Target = crate::System::Xml::Serialization::ObjectMap;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ListMap")]
impl std::ops::DerefMut for crate::System::Xml::Serialization::ListMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Serialization+ListMap")]
impl crate::System::Xml::Serialization::ListMap {
    pub fn set_ItemInfo(
        &mut self,
        value: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FindTextElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("FindTextElement", ())?;
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
    pub fn FindElement_Object_i32_Object0(
        &mut self,
        ob: *mut crate::System::Object,
        index: i32,
        memberValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("FindElement", (ob, index, memberValue))?;
        Ok(__cordl_ret)
    }
    pub fn FindElement_String_String1(
        &mut self,
        elementName: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfo = __cordl_object
            .invoke("FindElement", (elementName, ns))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        other: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
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
    pub fn get_ItemInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Serialization::XmlTypeMapElementInfoList = __cordl_object
            .invoke("get_ItemInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetArrayType(
        &mut self,
        itemCount: i32,
        localName: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        ns: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetArrayType", (itemCount, localName, ns))?;
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
#[cfg(feature = "System+Xml+Serialization+ListMap")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Serialization::ListMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Xml+Serialization+ListMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ListMap {
    __cordl_parent: crate::System::Xml::Serialization::ObjectMap,
    pub _itemInfo: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
    >,
    pub _choiceMember: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindElement_Il2CppObject_i32_Il2CppObject0(
        &mut self,
        ob: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        > = __cordl_object.invoke("FindElement", (ob, index, memberValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindElement_Il2CppString_Il2CppString1(
        &mut self,
        elementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        > = __cordl_object.invoke("FindElement", (elementName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindTextElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfo,
        > = __cordl_object.invoke("FindTextElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayType(
        &mut self,
        itemCount: i32,
        localName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        ns: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetArrayType", (itemCount, localName, ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
        > = __cordl_object.invoke("get_ItemInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ChoiceMember(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ChoiceMember", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlTypeMapElementInfoList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemInfo", (value))?;
        Ok(__cordl_ret.into())
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

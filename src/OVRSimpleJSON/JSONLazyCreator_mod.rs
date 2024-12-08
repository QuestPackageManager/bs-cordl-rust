#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
#[repr(C)]
#[derive(Debug)]
pub struct JSONLazyCreator {
    __cordl_parent: crate::OVRSimpleJSON::JSONNode,
    pub m_Node: *mut crate::OVRSimpleJSON::JSONNode,
    pub m_Key: *mut crate::System::String,
}
#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSONLazyCreator =>
    "OVRSimpleJSON"."JSONLazyCreator"
);
#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
impl std::ops::Deref for crate::OVRSimpleJSON::JSONLazyCreator {
    type Target = crate::OVRSimpleJSON::JSONNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
impl std::ops::DerefMut for crate::OVRSimpleJSON::JSONLazyCreator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
impl crate::OVRSimpleJSON::JSONLazyCreator {
    pub fn Add_JSONNode0(
        &mut self,
        aItem: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (aItem))?;
        Ok(__cordl_ret)
    }
    pub fn Add_String_JSONNode1(
        &mut self,
        aKey: *mut crate::System::String,
        aItem: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (aKey, aItem))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVRSimpleJSON::JSONNode_Enumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVRSimpleJSON::JSONNode_Enumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_JSONNode0(
        aNode: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aNode))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        aNode: *mut crate::OVRSimpleJSON::JSONNode,
        aKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aNode, aKey))?;
        Ok(__cordl_object)
    }
    pub fn Set<T>(&mut self, aVal: T) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Set", (aVal))?;
        Ok(__cordl_ret)
    }
    pub fn WriteToStringBuilder(
        &mut self,
        aSB: *mut crate::System::Text::StringBuilder,
        aIndent: i32,
        aIndentInc: i32,
        aMode: crate::OVRSimpleJSON::JSONTextMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToStringBuilder", (aSB, aIndent, aIndentInc, aMode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JSONNode0(
        &mut self,
        aNode: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aNode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        aNode: *mut crate::OVRSimpleJSON::JSONNode,
        aKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aNode, aKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_AsArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONArray> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONArray = __cordl_object
            .invoke("get_AsArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AsBool", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_AsDouble", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsFloat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AsFloat", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AsInt", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_AsLong", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AsObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONObject = __cordl_object
            .invoke("get_AsObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        aKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONNode = __cordl_object
            .invoke("get_Item", (aKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_0(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONNode = __cordl_object
            .invoke("get_Item", (aIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_Tag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::OVRSimpleJSON::JSONNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVRSimpleJSON::JSONNodeType = __cordl_object
            .invoke("get_Tag", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AsBool(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AsBool", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AsDouble(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AsDouble", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AsFloat(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AsFloat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AsInt(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AsInt", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_AsLong(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AsLong", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item_String1(
        &mut self,
        aKey: *mut crate::System::String,
        value: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aKey, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item_i32_0(
        &mut self,
        aIndex: i32,
        value: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aIndex, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRSimpleJSON+JSONLazyCreator")]
impl quest_hook::libil2cpp::ObjectType for crate::OVRSimpleJSON::JSONLazyCreator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

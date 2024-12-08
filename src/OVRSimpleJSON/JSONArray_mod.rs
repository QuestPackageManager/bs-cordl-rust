#[cfg(feature = "OVRSimpleJSON+JSONArray")]
#[repr(C)]
#[derive(Debug)]
pub struct JSONArray {
    __cordl_parent: crate::OVRSimpleJSON::JSONNode,
    pub m_List: *mut crate::System::Collections::Generic::List_1<
        *mut crate::OVRSimpleJSON::JSONNode,
    >,
    pub _cordl_inline: bool,
}
#[cfg(feature = "OVRSimpleJSON+JSONArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSONArray => "OVRSimpleJSON"
    ."JSONArray"
);
#[cfg(feature = "OVRSimpleJSON+JSONArray")]
impl std::ops::Deref for crate::OVRSimpleJSON::JSONArray {
    type Target = crate::OVRSimpleJSON::JSONNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSONArray")]
impl std::ops::DerefMut for crate::OVRSimpleJSON::JSONArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSONArray")]
impl crate::OVRSimpleJSON::JSONArray {
    #[cfg(feature = "OVRSimpleJSON+JSONArray+_get_Children_d__22")]
    pub type _get_Children_d__22 = crate::OVRSimpleJSON::JSONArray__get_Children_d__22;
    pub fn Add(
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Remove_JSONNode1(
        &mut self,
        aNode: *mut crate::OVRSimpleJSON::JSONNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONNode = __cordl_object
            .invoke("Remove", (aNode))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_i32_0(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::OVRSimpleJSON::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::OVRSimpleJSON::JSONNode = __cordl_object
            .invoke("Remove", (aIndex))?;
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
    pub fn get_Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::OVRSimpleJSON::JSONNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::OVRSimpleJSON::JSONNode,
        > = __cordl_object.invoke("get_Children", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Inline(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Inline", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsArray", ())?;
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
    pub fn set_Inline(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Inline", (value))?;
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
#[cfg(feature = "OVRSimpleJSON+JSONArray")]
impl quest_hook::libil2cpp::ObjectType for crate::OVRSimpleJSON::JSONArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

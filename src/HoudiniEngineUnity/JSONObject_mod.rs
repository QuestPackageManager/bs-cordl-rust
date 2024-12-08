#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
#[repr(C)]
#[derive(Debug)]
pub struct JSONObject {
    __cordl_parent: crate::HoudiniEngineUnity::JSONNode,
    pub m_Dict: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::HoudiniEngineUnity::JSONNode,
    >,
    pub _cordl_inline: bool,
}
#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONObject =>
    "HoudiniEngineUnity"."JSONObject"
);
#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
impl std::ops::Deref for crate::HoudiniEngineUnity::JSONObject {
    type Target = crate::HoudiniEngineUnity::JSONNode;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::JSONObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
impl crate::HoudiniEngineUnity::JSONObject {
    #[cfg(feature = "HoudiniEngineUnity+JSONObject+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::HoudiniEngineUnity::JSONObject___c__DisplayClass21_0;
    #[cfg(feature = "HoudiniEngineUnity+JSONObject+_get_Children_d__23")]
    pub type _get_Children_d__23 = crate::HoudiniEngineUnity::JSONObject__get_Children_d__23;
    pub fn Add(
        &mut self,
        aKey: *mut crate::System::String,
        aItem: *mut crate::HoudiniEngineUnity::JSONNode,
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
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::JSONNode_Enumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_Enumerator = __cordl_object
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
    pub fn Remove_JSONNode2(
        &mut self,
        aNode: *mut crate::HoudiniEngineUnity::JSONNode,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::JSONNode = __cordl_object
            .invoke("Remove", (aNode))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_String0(
        &mut self,
        aKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::JSONNode = __cordl_object
            .invoke("Remove", (aKey))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_i32_1(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::JSONNode = __cordl_object
            .invoke("Remove", (aIndex))?;
        Ok(__cordl_ret)
    }
    pub fn WriteToStringBuilder(
        &mut self,
        aSB: *mut crate::System::Text::StringBuilder,
        aIndent: i32,
        aIndentInc: i32,
        aMode: crate::HoudiniEngineUnity::JSONTextMode,
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
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::HoudiniEngineUnity::JSONNode,
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
    pub fn get_IsObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String0(
        &mut self,
        aKey: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::JSONNode = __cordl_object
            .invoke("get_Item", (aKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_1(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::JSONNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::JSONNode = __cordl_object
            .invoke("get_Item", (aIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_Tag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::JSONNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNodeType = __cordl_object
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
    pub fn set_Item_String0(
        &mut self,
        aKey: *mut crate::System::String,
        value: *mut crate::HoudiniEngineUnity::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aKey, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item_i32_1(
        &mut self,
        aIndex: i32,
        value: *mut crate::HoudiniEngineUnity::JSONNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aIndex, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONObject")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::JSONObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

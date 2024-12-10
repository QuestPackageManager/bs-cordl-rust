#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enumerator_JSONNode_Type {
    Array = 1i32,
    None = 0i32,
    Object = 2i32,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::Enumerator_JSONNode_Type =>
    "HoudiniEngineUnity"."JSONNode/Enumerator/Type"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
#[repr(C)]
#[derive(Debug)]
pub struct JSONNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONNode =>
    "HoudiniEngineUnity"."JSONNode"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
impl std::ops::Deref for crate::HoudiniEngineUnity::JSONNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::JSONNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
impl crate::HoudiniEngineUnity::JSONNode {
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator")]
    pub type Enumerator = crate::HoudiniEngineUnity::JSONNode_Enumerator;
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+KeyEnumerator")]
    pub type KeyEnumerator = crate::HoudiniEngineUnity::JSONNode_KeyEnumerator;
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
    pub type LinqEnumerator = crate::HoudiniEngineUnity::JSONNode_LinqEnumerator;
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+ValueEnumerator")]
    pub type ValueEnumerator = crate::HoudiniEngineUnity::JSONNode_ValueEnumerator;
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+_get_Children_d__40")]
    pub type _get_Children_d__40 = crate::HoudiniEngineUnity::JSONNode__get_Children_d__40;
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+_get_DeepChildren_d__42")]
    pub type _get_DeepChildren_d__42 = crate::HoudiniEngineUnity::JSONNode__get_DeepChildren_d__42;
    pub fn Add_Il2CppString_JSONNode0(
        &mut self,
        aKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aItem: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (aKey, aItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_JSONNode1(
        &mut self,
        aItem: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (aItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::JSONNode_Enumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_Enumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
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
    pub fn ReadMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("ReadMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuaternion_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("ReadQuaternion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadQuaternion_Quaternion0(
        &mut self,
        aDefault: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("ReadQuaternion", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRectOffset_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset> = __cordl_object
            .invoke("ReadRectOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRectOffset_RectOffset0(
        &mut self,
        aDefault: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset> = __cordl_object
            .invoke("ReadRectOffset", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRect_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("ReadRect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadRect_Rect0(
        &mut self,
        aDefault: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("ReadRect", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector2_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ReadVector2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector2_Il2CppString_Il2CppString1(
        &mut self,
        aXName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aYName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ReadVector2", (aXName, aYName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector2_Vector2_0(
        &mut self,
        aDefault: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ReadVector2", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ReadVector3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3_Il2CppString_Il2CppString_Il2CppString1(
        &mut self,
        aXName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aYName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aZName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ReadVector3", (aXName, aYName, aZName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector3_Vector3_0(
        &mut self,
        aDefault: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("ReadVector3", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector4_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("ReadVector4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVector4_Vector4_0(
        &mut self,
        aDefault: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("ReadVector4", (aDefault))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_Il2CppString0(
        &mut self,
        aKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("Remove", (aKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_JSONNode2(
        &mut self,
        aNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("Remove", (aNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_i32_1(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("Remove", (aIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_1(
        &mut self,
        aIndent: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (aIndent))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteMatrix(
        &mut self,
        aMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteMatrix", (aMatrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteQuaternion(
        &mut self,
        aRot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteQuaternion", (aRot))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRect(
        &mut self,
        aRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteRect", (aRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteRectOffset(
        &mut self,
        aRect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectOffset>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteRectOffset", (aRect))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToStringBuilder(
        &mut self,
        aSB: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        aIndent: i32,
        aIndentInc: i32,
        aMode: crate::HoudiniEngineUnity::JSONTextMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteToStringBuilder", (aSB, aIndent, aIndentInc, aMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteVector2(
        &mut self,
        aVec: crate::UnityEngine::Vector2,
        aXName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aYName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteVector2", (aVec, aXName, aYName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteVector3(
        &mut self,
        aVec: crate::UnityEngine::Vector3,
        aXName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aYName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        aZName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteVector3", (aVec, aXName, aYName, aZName))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteVector4(
        &mut self,
        aVec: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("WriteVector4", (aVec))?;
        Ok(__cordl_ret.into())
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
    pub fn get_AsArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONArray>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONArray,
        > = __cordl_object.invoke("get_AsArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AsBool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_AsDouble", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsFloat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AsFloat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AsInt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_AsLong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AsObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONObject,
        > = __cordl_object.invoke("get_AsObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::HoudiniEngineUnity::JSONNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::HoudiniEngineUnity::JSONNode,
            >,
        > = __cordl_object.invoke("get_Children", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DeepChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::HoudiniEngineUnity::JSONNode,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::HoudiniEngineUnity::JSONNode,
            >,
        > = __cordl_object.invoke("get_DeepChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Inline(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Inline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsArray(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBoolean(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsBoolean", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNumber(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsObject(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsString(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_Il2CppString1(
        &mut self,
        aKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("get_Item", (aKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_i32_0(
        &mut self,
        aIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("get_Item", (aIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::JSONNode_KeyEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_KeyEnumerator = __cordl_object
            .invoke("get_Keys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Linq(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::HoudiniEngineUnity::JSONNode,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::HoudiniEngineUnity::JSONNode,
                >,
            >,
        > = __cordl_object.invoke("get_Linq", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Tag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::JSONNodeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNodeType = __cordl_object
            .invoke("get_Tag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::JSONNode_ValueEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_ValueEnumerator = __cordl_object
            .invoke("get_Values", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_Item_Il2CppString1(
        &mut self,
        aKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aKey, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item_i32_0(
        &mut self,
        aIndex: i32,
        value: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (aIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::JSONNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JSONNode_Enumerator {
    pub _cordl_type: crate::HoudiniEngineUnity::Enumerator_JSONNode_Type,
    pub m_Object: crate::System::Collections::Generic::Dictionary_2_Enumerator<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::HoudiniEngineUnity::JSONNode,
    >,
    pub m_Array: crate::System::Collections::Generic::List_1_Enumerator<
        *mut crate::HoudiniEngineUnity::JSONNode,
    >,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONNode_Enumerator =>
    "HoudiniEngineUnity"."JSONNode/Enumerator"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::JSONNode_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator")]
impl crate::HoudiniEngineUnity::JSONNode_Enumerator {
    #[cfg(feature = "HoudiniEngineUnity+JSONNode+Enumerator+Type")]
    pub type Type = crate::HoudiniEngineUnity::Enumerator_JSONNode_Type;
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Dictionary_2_Enumerator1(
        &mut self,
        aDictEnum: crate::System::Collections::Generic::Dictionary_2_Enumerator<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aDictEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_Enumerator0(
        &mut self,
        aArrayEnum: crate::System::Collections::Generic::List_1_Enumerator<
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aArrayEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        let __cordl_ret: crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+KeyEnumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JSONNode_KeyEnumerator {
    pub m_Enumerator: crate::HoudiniEngineUnity::JSONNode_Enumerator,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+KeyEnumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONNode_KeyEnumerator =>
    "HoudiniEngineUnity"."JSONNode/KeyEnumerator"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode+KeyEnumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::JSONNode_KeyEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+KeyEnumerator")]
impl crate::HoudiniEngineUnity::JSONNode_KeyEnumerator {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::JSONNode_KeyEnumerator,
    > {
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_KeyEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Dictionary_2_Enumerator1(
        &mut self,
        aDictEnum: crate::System::Collections::Generic::Dictionary_2_Enumerator<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aDictEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_JSONNode_Enumerator2(
        &mut self,
        aEnumerator: crate::HoudiniEngineUnity::JSONNode_Enumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aEnumerator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_Enumerator0(
        &mut self,
        aArrayEnum: crate::System::Collections::Generic::List_1_Enumerator<
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aArrayEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct JSONNode_LinqEnumerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Node: *mut crate::HoudiniEngineUnity::JSONNode,
    pub m_Enumerator: crate::HoudiniEngineUnity::JSONNode_Enumerator,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONNode_LinqEnumerator =>
    "HoudiniEngineUnity"."JSONNode/LinqEnumerator"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl std::ops::Deref for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::HoudiniEngineUnity::JSONNode,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::HoudiniEngineUnity::JSONNode,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aNode))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        aNode: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        > = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    >,
> for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    >,
> for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    >,
> for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    >,
> for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsRef<crate::System::IDisposable>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+LinqEnumerator")]
impl AsMut<crate::System::IDisposable>
for crate::HoudiniEngineUnity::JSONNode_LinqEnumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+ValueEnumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JSONNode_ValueEnumerator {
    pub m_Enumerator: crate::HoudiniEngineUnity::JSONNode_Enumerator,
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+ValueEnumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::JSONNode_ValueEnumerator =>
    "HoudiniEngineUnity"."JSONNode/ValueEnumerator"
);
#[cfg(feature = "HoudiniEngineUnity+JSONNode+ValueEnumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::JSONNode_ValueEnumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+JSONNode+ValueEnumerator")]
impl crate::HoudiniEngineUnity::JSONNode_ValueEnumerator {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::HoudiniEngineUnity::JSONNode_ValueEnumerator,
    > {
        let __cordl_ret: crate::HoudiniEngineUnity::JSONNode_ValueEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Dictionary_2_Enumerator1(
        &mut self,
        aDictEnum: crate::System::Collections::Generic::Dictionary_2_Enumerator<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aDictEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_JSONNode_Enumerator2(
        &mut self,
        aEnumerator: crate::HoudiniEngineUnity::JSONNode_Enumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aEnumerator),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_Enumerator0(
        &mut self,
        aArrayEnum: crate::System::Collections::Generic::List_1_Enumerator<
            *mut crate::HoudiniEngineUnity::JSONNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (aArrayEnum),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::JSONNode>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::JSONNode,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}

#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct TextHandle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_PreferredSize: crate::UnityEngine::Vector2,
    pub m_TextInfo: *mut crate::UnityEngine::TextCore::Text::TextInfo,
    pub m_PreviousGenerationSettingsHash: i32,
    pub textGenerationSettings: *mut crate::UnityEngine::TextCore::Text::TextGenerationSettings,
    pub isDirty: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextHandle =>
    "UnityEngine.TextCore.Text"."TextHandle"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextHandle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
impl crate::UnityEngine::TextCore::Text::TextHandle {
    pub fn ComputeTextHeight(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ComputeTextHeight", (tgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextWidth(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ComputeTextWidth", (tgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn DistanceToLine(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DistanceToLine", (a, b, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLink(
        &mut self,
        position: crate::UnityEngine::Vector3,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindIntersectingLink", (position, inverseYAxis))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacterOnLine(
        &mut self,
        position: crate::UnityEngine::Vector2,
        line: i32,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindNearestCharacterOnLine", (position, line, visibleOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLine(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNearestLine", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacterHeightFromIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetCharacterHeightFromIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition(
        &mut self,
        position: crate::UnityEngine::Vector2,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCursorIndexFromPosition", (position, inverseYAxis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorPositionFromStringIndexUsingCharacterHeight(
        &mut self,
        index: i32,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke(
                "GetCursorPositionFromStringIndexUsingCharacterHeight",
                (index, inverseYAxis),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorPositionFromStringIndexUsingLineHeight(
        &mut self,
        index: i32,
        useXAdvance: bool,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke(
                "GetCursorPositionFromStringIndexUsingLineHeight",
                (index, useXAdvance, inverseYAxis),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLineHeight(
        &mut self,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLineHeight", (lineNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLineHeightFromCharacterIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetLineHeightFromCharacterIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLineNumber(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLineNumber", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        value: char,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsElided(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsElided", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf(
        &mut self,
        value: char,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LineDownCharacterPosition(
        &mut self,
        originalPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LineDownCharacterPosition", (originalPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn LineUpCharacterPosition(
        &mut self,
        originalPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LineUpCharacterPosition", (originalPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PointIntersectRectangle(
        m: crate::UnityEngine::Vector3,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        c: crate::UnityEngine::Vector3,
        d: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointIntersectRectangle", (m, a, b, c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Substring(
        &mut self,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Substring", (startIndex, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = __cordl_object.invoke("Update", (tgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePreferredValues(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePreferredValues", (tgs))?;
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
    pub fn get_layoutTextInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_layoutTextInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = __cordl_object.invoke("get_textInfo", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

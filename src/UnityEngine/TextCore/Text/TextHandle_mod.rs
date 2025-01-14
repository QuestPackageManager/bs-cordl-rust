#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct TextHandle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_PreferredSize: crate::UnityEngine::Vector2,
    pub m_TextInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextInfo,
    >,
    pub m_PreviousGenerationSettingsHash: i32,
    pub textGenerationSettings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextGenerationSettings,
    >,
    pub isDirty: bool,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextHandle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextHandle";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                f32,
                1usize,
            >("ComputeTextHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeTextHeight", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (tgs)) };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeTextWidth(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                f32,
                1usize,
            >("ComputeTextWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeTextWidth", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (tgs)) };
        Ok(__cordl_ret.into())
    }
    pub fn DistanceToLine(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                f32,
                3usize,
            >("DistanceToLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DistanceToLine", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b, point)) };
        Ok(__cordl_ret.into())
    }
    pub fn FindIntersectingLink(
        &mut self,
        position: crate::UnityEngine::Vector3,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3, bool),
                i32,
                2usize,
            >("FindIntersectingLink")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindIntersectingLink", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (position, inverseYAxis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestCharacterOnLine(
        &mut self,
        position: crate::UnityEngine::Vector2,
        line: i32,
        visibleOnly: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2, i32, bool),
                i32,
                3usize,
            >("FindNearestCharacterOnLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindNearestCharacterOnLine", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (position, line, visibleOnly))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindNearestLine(
        &mut self,
        position: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::UnityEngine::Vector2), i32, 1usize>("FindNearestLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindNearestLine", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (position)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacterHeightFromIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetCharacterHeightFromIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCharacterHeightFromIndex", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorIndexFromPosition(
        &mut self,
        position: crate::UnityEngine::Vector2,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2, bool),
                i32,
                2usize,
            >("GetCursorIndexFromPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCursorIndexFromPosition", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (position, inverseYAxis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorPositionFromStringIndexUsingCharacterHeight(
        &mut self,
        index: i32,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool),
                crate::UnityEngine::Vector2,
                2usize,
            >("GetCursorPositionFromStringIndexUsingCharacterHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCursorPositionFromStringIndexUsingCharacterHeight", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (index, inverseYAxis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorPositionFromStringIndexUsingLineHeight(
        &mut self,
        index: i32,
        useXAdvance: bool,
        inverseYAxis: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, bool, bool),
                crate::UnityEngine::Vector2,
                3usize,
            >("GetCursorPositionFromStringIndexUsingLineHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCursorPositionFromStringIndexUsingLineHeight", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, (index, useXAdvance, inverseYAxis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineHeight(
        &mut self,
        lineNumber: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetLineHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLineHeight", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (lineNumber)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineHeightFromCharacterIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetLineHeightFromCharacterIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLineHeightFromCharacterIndex", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLineNumber(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetLineNumber")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLineNumber", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        value: char,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(char, i32), i32, 2usize>("IndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IndexOf", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (value, startIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsDirty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsDirty", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn IsElided(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsElided")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsElided", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf(
        &mut self,
        value: char,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(char, i32), i32, 2usize>("LastIndexOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LastIndexOf", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (value, startIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LineDownCharacterPosition(
        &mut self,
        originalPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("LineDownCharacterPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LineDownCharacterPosition", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (originalPos)) };
        Ok(__cordl_ret.into())
    }
    pub fn LineUpCharacterPosition(
        &mut self,
        originalPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("LineUpCharacterPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LineUpCharacterPosition", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (originalPos)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                ),
                bool,
                5usize,
            >("PointIntersectRectangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointIntersectRectangle", 5usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (m, a, b, c, d)) };
        Ok(__cordl_ret.into())
    }
    pub fn Substring(
        &mut self,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("Substring")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Substring", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (startIndex, length)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
                1usize,
            >("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = unsafe { method.invoke_unchecked(self, (tgs)) };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePreferredValues(
        &mut self,
        tgs: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextGenerationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::TextCore::Text::TextGenerationSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdatePreferredValues")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdatePreferredValues", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tgs))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutTextInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
                0usize,
            >("get_layoutTextInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_layoutTextInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_textInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Text::TextInfo>,
                0usize,
            >("get_textInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_textInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        > = unsafe { method.invoke_unchecked(self, ()) };
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

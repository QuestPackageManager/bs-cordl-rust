#[cfg(feature = "UnityEngine+TextGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Ptr: crate::System::IntPtr,
    pub m_LastString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_LastSettings: crate::UnityEngine::TextGenerationSettings,
    pub m_HasGenerated: bool,
    pub m_LastValid: crate::UnityEngine::TextGenerationError,
    pub m_Verts: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIVertex>,
    pub m_Characters: quest_hook::libil2cpp::Gc<crate::UnityEngine::UICharInfo>,
    pub m_Lines: quest_hook::libil2cpp::Gc<crate::UnityEngine::UILineInfo>,
    pub m_CachedVerts: bool,
    pub m_CachedCharacters: bool,
    pub m_CachedLines: bool,
}
#[cfg(feature = "UnityEngine+TextGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextGenerator => "UnityEngine"
    ."TextGenerator"
);
#[cfg(feature = "UnityEngine+TextGenerator")]
impl std::ops::Deref for crate::UnityEngine::TextGenerator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextGenerator")]
impl std::ops::DerefMut for crate::UnityEngine::TextGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextGenerator")]
impl crate::UnityEngine::TextGenerator {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacters(
        &mut self,
        characters: quest_hook::libil2cpp::Gc<crate::UnityEngine::UICharInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCharacters", (characters))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharactersInternal(
        &mut self,
        characters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCharactersInternal", (characters))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLines(
        &mut self,
        lines: quest_hook::libil2cpp::Gc<crate::UnityEngine::UILineInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLines", (lines))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLinesInternal(
        &mut self,
        lines: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLinesInternal", (lines))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredHeight(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetPreferredHeight", (str, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPreferredWidth(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetPreferredWidth", (str, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertices(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIVertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertices", (vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVerticesInternal(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVerticesInternal", (vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Destroy(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Destroy", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn Populate(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Populate", (str, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateAlways(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextGenerationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextGenerationError = __cordl_object
            .invoke("PopulateAlways", (str, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateWithError(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextGenerationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextGenerationError = __cordl_object
            .invoke("PopulateWithError", (str, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateWithErrors(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: crate::UnityEngine::TextGenerationSettings,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PopulateWithErrors", (str, settings, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Populate_Internal_Injected(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        fontSize: i32,
        scaleFactor: f32,
        lineSpacing: f32,
        style: crate::UnityEngine::FontStyle,
        richText: bool,
        resizeTextForBestFit: bool,
        resizeTextMinSize: i32,
        resizeTextMaxSize: i32,
        verticalOverFlow: i32,
        horizontalOverflow: i32,
        updateBounds: bool,
        anchor: crate::UnityEngine::TextAnchor,
        extentsX: f32,
        extentsY: f32,
        pivotX: f32,
        pivotY: f32,
        generateOutOfBounds: bool,
        alignByGeometry: bool,
        error: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Populate_Internal_Injected",
                (
                    str,
                    font,
                    color,
                    fontSize,
                    scaleFactor,
                    lineSpacing,
                    style,
                    richText,
                    resizeTextForBestFit,
                    resizeTextMinSize,
                    resizeTextMaxSize,
                    verticalOverFlow,
                    horizontalOverflow,
                    updateBounds,
                    anchor,
                    extentsX,
                    extentsY,
                    pivotX,
                    pivotY,
                    generateOutOfBounds,
                    alignByGeometry,
                    error,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Populate_Internal_VerticalWrapMode_HorizontalWrapMode_Vector2_Vector2__cordl_bool__cordl_bool_ByRefMut1(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        color: crate::UnityEngine::Color,
        fontSize: i32,
        scaleFactor: f32,
        lineSpacing: f32,
        style: crate::UnityEngine::FontStyle,
        richText: bool,
        resizeTextForBestFit: bool,
        resizeTextMinSize: i32,
        resizeTextMaxSize: i32,
        verticalOverFlow: crate::UnityEngine::VerticalWrapMode,
        horizontalOverflow: crate::UnityEngine::HorizontalWrapMode,
        updateBounds: bool,
        anchor: crate::UnityEngine::TextAnchor,
        extents: crate::UnityEngine::Vector2,
        pivot: crate::UnityEngine::Vector2,
        generateOutOfBounds: bool,
        alignByGeometry: bool,
        error: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::TextGenerationError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Populate_Internal",
                (
                    str,
                    font,
                    color,
                    fontSize,
                    scaleFactor,
                    lineSpacing,
                    style,
                    richText,
                    resizeTextForBestFit,
                    resizeTextMinSize,
                    resizeTextMaxSize,
                    verticalOverFlow,
                    horizontalOverflow,
                    updateBounds,
                    anchor,
                    extents,
                    pivot,
                    generateOutOfBounds,
                    alignByGeometry,
                    error,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Populate_Internal_i32_i32_f32_f32_f32_f32__cordl_bool__cordl_bool_ByRefMut0(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        color: crate::UnityEngine::Color,
        fontSize: i32,
        scaleFactor: f32,
        lineSpacing: f32,
        style: crate::UnityEngine::FontStyle,
        richText: bool,
        resizeTextForBestFit: bool,
        resizeTextMinSize: i32,
        resizeTextMaxSize: i32,
        verticalOverFlow: i32,
        horizontalOverflow: i32,
        updateBounds: bool,
        anchor: crate::UnityEngine::TextAnchor,
        extentsX: f32,
        extentsY: f32,
        pivotX: f32,
        pivotY: f32,
        generateOutOfBounds: bool,
        alignByGeometry: bool,
        error: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Populate_Internal",
                (
                    str,
                    font,
                    color,
                    fontSize,
                    scaleFactor,
                    lineSpacing,
                    style,
                    richText,
                    resizeTextForBestFit,
                    resizeTextMinSize,
                    resizeTextMaxSize,
                    verticalOverFlow,
                    horizontalOverflow,
                    updateBounds,
                    anchor,
                    extentsX,
                    extentsY,
                    pivotX,
                    pivotY,
                    generateOutOfBounds,
                    alignByGeometry,
                    error,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidatedSettings(
        &mut self,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextGenerationSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextGenerationSettings = __cordl_object
            .invoke("ValidatedSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        initialCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialCapacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_characterCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_characterCountVisible(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterCountVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_characters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UICharInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UICharInfo> = __cordl_object
            .invoke("get_characters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lines(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UILineInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UILineInfo> = __cordl_object
            .invoke("get_lines", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectExtents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rectExtents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectExtents_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_rectExtents_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_verts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIVertex>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIVertex> = __cordl_object
            .invoke("get_verts", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextGenerator")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TextGenerator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::TextGenerator {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+TextGenerator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::TextGenerator {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}

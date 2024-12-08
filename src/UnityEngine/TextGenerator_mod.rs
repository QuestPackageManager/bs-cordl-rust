#[cfg(feature = "UnityEngine+TextGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TextGenerator {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
    pub m_LastString: *mut crate::System::String,
    pub m_LastSettings: crate::UnityEngine::TextGenerationSettings,
    pub m_HasGenerated: bool,
    pub m_LastValid: crate::UnityEngine::TextGenerationError,
    pub m_Verts: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIVertex,
    >,
    pub m_Characters: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UICharInfo,
    >,
    pub m_Lines: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UILineInfo,
    >,
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
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn GetCharacters(
        &mut self,
        characters: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UICharInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCharacters", (characters))?;
        Ok(__cordl_ret)
    }
    pub fn GetCharactersInternal(
        &mut self,
        characters: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCharactersInternal", (characters))?;
        Ok(__cordl_ret)
    }
    pub fn GetLines(
        &mut self,
        lines: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UILineInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLines", (lines))?;
        Ok(__cordl_ret)
    }
    pub fn GetLinesInternal(
        &mut self,
        lines: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLinesInternal", (lines))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredHeight(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetPreferredHeight", (str, settings))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreferredWidth(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetPreferredWidth", (str, settings))?;
        Ok(__cordl_ret)
    }
    pub fn GetVertices(
        &mut self,
        vertices: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIVertex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertices", (vertices))?;
        Ok(__cordl_ret)
    }
    pub fn GetVerticesInternal(
        &mut self,
        vertices: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVerticesInternal", (vertices))?;
        Ok(__cordl_ret)
    }
    pub fn Invalidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invalidate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(initialCapacity: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialCapacity))?;
        Ok(__cordl_object)
    }
    pub fn Populate(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Populate", (str, settings))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateAlways(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextGenerationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextGenerationError = __cordl_object
            .invoke("PopulateAlways", (str, settings))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateWithError(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextGenerationError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextGenerationError = __cordl_object
            .invoke("PopulateWithError", (str, settings))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateWithErrors(
        &mut self,
        str: *mut crate::System::String,
        settings: crate::UnityEngine::TextGenerationSettings,
        context: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PopulateWithErrors", (str, settings, context))?;
        Ok(__cordl_ret)
    }
    pub fn Populate_Internal_Injected(
        &mut self,
        str: *mut crate::System::String,
        font: *mut crate::UnityEngine::Font,
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
        Ok(__cordl_ret)
    }
    pub fn Populate_Internal_VerticalWrapMode_HorizontalWrapMode_Vector2_Vector2__cordl_bool__cordl_bool_ByRefMut1(
        &mut self,
        str: *mut crate::System::String,
        font: *mut crate::UnityEngine::Font,
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
        Ok(__cordl_ret)
    }
    pub fn Populate_Internal_i32_i32_f32_f32_f32_f32__cordl_bool__cordl_bool_ByRefMut0(
        &mut self,
        str: *mut crate::System::String,
        font: *mut crate::UnityEngine::Font,
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
        Ok(__cordl_ret)
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.IDisposable.Dispose", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_characterCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterCountVisible(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterCountVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<crate::UnityEngine::UICharInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::UICharInfo,
        > = __cordl_object.invoke("get_characters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lines(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<crate::UnityEngine::UILineInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::UILineInfo,
        > = __cordl_object.invoke("get_lines", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rectExtents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_rectExtents", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_verts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<crate::UnityEngine::UIVertex>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::UIVertex,
        > = __cordl_object.invoke("get_verts", ())?;
        Ok(__cordl_ret)
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
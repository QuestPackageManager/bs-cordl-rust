#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "SelectionPickerRenderer";
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
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    pub const k_FacePickerOcclusionTintUniform: &'static str = "_Tint";
    pub const k_MinEdgePixelsForValidSelection: u32 = 16843009u32;
    pub const k_PickerHashMax: u32 = 4294967232u32;
    pub const k_PickerHashMin: u32 = 4294950913u32;
    pub const k_PickerHashNone: u32 = 4290773248u32;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
    )]
    type ISelectionPickerRenderer = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
    )]
    pub type SelectionPickerRendererHDRP = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
    )]
    pub type SelectionPickerRendererStandard = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard;
    pub fn BuildEdgeMesh(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        map: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    crate::UnityEngine::ProBuilder::Edge,
                >,
            >,
        >,
        index: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildEdgeMesh", (pb, map, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildVertexMesh(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        map: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                u32,
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    i32,
                >,
            >,
        >,
        index: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildVertexMesh", (pb, map, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeRGBA(
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecodeRGBA", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeRGBA(
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EncodeRGBA", (hash))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateEdgePickingObjects(
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        >,
        depthObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                >,
            >,
        >,
        pickerObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateEdgePickingObjects",
                (selection, doDepthTest, map, depthObjects, pickerObjects),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateFacePickingObjects(
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateFacePickingObjects", (selection, map))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateVertexPickingObjects(
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        i32,
                    >,
                >,
            >,
        >,
        depthObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                >,
            >,
        >,
        pickerObjects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateVertexPickingObjects",
                (selection, doDepthTest, map, depthObjects, pickerObjects),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickEdgesInRect(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        pickerRect: crate::UnityEngine::Rect,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        renderTextureWidth: i32,
        renderTextureHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickEdgesInRect",
                (
                    camera,
                    pickerRect,
                    selection,
                    doDepthTest,
                    renderTextureWidth,
                    renderTextureHeight,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickFacesInRect(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        pickerRect: crate::UnityEngine::Rect,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        renderTextureWidth: i32,
        renderTextureHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickFacesInRect",
                (camera, pickerRect, selection, renderTextureWidth, renderTextureHeight),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PickVerticesInRect(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        pickerRect: crate::UnityEngine::Rect,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        renderTextureWidth: i32,
        renderTextureHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<i32>,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<i32>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickVerticesInRect",
                (
                    camera,
                    pickerRect,
                    selection,
                    doDepthTest,
                    renderTextureWidth,
                    renderTextureHeight,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderSelectionPickerTexture_ByRefMut_i32_0(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                    >,
                >,
            >,
        >,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RenderSelectionPickerTexture",
                (camera, selection, map, width, height),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderSelectionPickerTexture__cordl_bool_ByRefMut_i32_1(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        i32,
                    >,
                >,
            >,
        >,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RenderSelectionPickerTexture",
                (camera, selection, doDepthTest, map, width, height),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderSelectionPickerTexture__cordl_bool_ByRefMut_i32_2(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        selection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
            >,
        >,
        doDepthTest: bool,
        map: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    u32,
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::ProBuilderMesh,
                        >,
                        crate::UnityEngine::ProBuilder::Edge,
                    >,
                >,
            >,
        >,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RenderSelectionPickerTexture",
                (camera, selection, doDepthTest, map, width, height),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldUseHDRP() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldUseHDRP", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pickerRenderer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pickerRenderer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderTextureFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::RenderTextureFormat,
    > {
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderTextureFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textureFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextureFormat,
    > {
        let __cordl_ret: crate::UnityEngine::TextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_textureFormat", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_ISelectionPickerRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "ISelectionPickerRenderer";
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
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    pub fn RenderLookupTexture(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_SelectionPickerRendererHDRP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "SelectionPickerRendererHDRP";
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
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RenderLookupTexture(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
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
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl AsRef<
    crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
>
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl AsMut<
    crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
>
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_SelectionPickerRendererStandard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "SelectionPickerRendererStandard";
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
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RenderLookupTexture(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        tag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
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
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl AsRef<
    crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
>
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl AsMut<
    crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
>
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
        unsafe { std::mem::transmute(self) }
    }
}

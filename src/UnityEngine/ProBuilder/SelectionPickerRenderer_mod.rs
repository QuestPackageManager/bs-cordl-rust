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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    pub const k_FacePickerOcclusionTintUniform: &'static str = "_Tint";
    pub const k_MinEdgePixelsForValidSelection: u32 = 1u32;
    pub const k_PickerHashMax: u32 = 16777215u32;
    pub const k_PickerHashMin: u32 = 1u32;
    pub const k_PickerHashNone: u32 = 0u32;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
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
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        3usize,
                    >("BuildEdgeMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildEdgeMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = unsafe {
            cordl_method_info.invoke_unchecked((), (pb, map, index))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                            >,
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
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        3usize,
                    >("BuildVertexMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildVertexMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = unsafe {
            cordl_method_info.invoke_unchecked((), (pb, map, index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecodeRGBA(
        color: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Color32),
                        u32,
                        1usize,
                    >("DecodeRGBA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecodeRGBA", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EncodeRGBA(
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("EncodeRGBA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EncodeRGBA", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked((), (hash))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("GenerateEdgePickingObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateEdgePickingObjects", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (selection, doDepthTest, map, depthObjects, pickerObjects),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        u32,
                                        crate::UnityEngine::ProBuilder::SimpleTuple_2<
                                            quest_hook::libil2cpp::Gc<
                                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                            >,
                                            quest_hook::libil2cpp::Gc<
                                                crate::UnityEngine::ProBuilder::Face,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            >,
                        >,
                        2usize,
                    >("GenerateFacePickingObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateFacePickingObjects", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (selection, map))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("GenerateVertexPickingObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateVertexPickingObjects", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (selection, doDepthTest, map, depthObjects, pickerObjects),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            i32,
                            i32,
                        ),
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
                        6usize,
                    >("PickEdgesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickEdgesInRect", 6usize
                        )
                    })
            });
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
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        pickerRect,
                        selection,
                        doDepthTest,
                        renderTextureWidth,
                        renderTextureHeight,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::HashSet_1<
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::ProBuilder::Face,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                        5usize,
                    >("PickFacesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickFacesInRect", 5usize
                        )
                    })
            });
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
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        pickerRect,
                        selection,
                        renderTextureWidth,
                        renderTextureHeight,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::Rect,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            i32,
                            i32,
                        ),
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
                        6usize,
                    >("PickVerticesInRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PickVerticesInRect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::ProBuilderMesh,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<i32>,
                >,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        camera,
                        pickerRect,
                        selection,
                        doDepthTest,
                        renderTextureWidth,
                        renderTextureHeight,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::Dictionary_2<
                                        u32,
                                        crate::UnityEngine::ProBuilder::SimpleTuple_2<
                                            quest_hook::libil2cpp::Gc<
                                                crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                            >,
                                            quest_hook::libil2cpp::Gc<
                                                crate::UnityEngine::ProBuilder::Face,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        5usize,
                    >("RenderSelectionPickerTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderSelectionPickerTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (camera, selection, map, width, height))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
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
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        6usize,
                    >("RenderSelectionPickerTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderSelectionPickerTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (camera, selection, doDepthTest, map, width, height),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
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
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        6usize,
                    >("RenderSelectionPickerTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderSelectionPickerTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (camera, selection, doDepthTest, map, width, height),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldUseHDRP() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("ShouldUseHDRP")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShouldUseHDRP", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pickerRenderer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
                        >,
                        0usize,
                    >("get_pickerRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pickerRenderer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderTextureFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::RenderTextureFormat,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::RenderTextureFormat,
                        0usize,
                    >("get_renderTextureFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_renderTextureFormat", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_textureFormat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextureFormat,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::TextureFormat,
                        0usize,
                    >("get_textureFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_textureFormat", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextureFormat = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
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
    const CLASS_NAME: &'static str = "SelectionPickerRenderer/ISelectionPickerRenderer";
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        5usize,
                    >("RenderLookupTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderLookupTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (camera, shader, tag, width, height))?
        };
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
    const CLASS_NAME: &'static str = "SelectionPickerRenderer/SelectionPickerRendererHDRP";
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        5usize,
                    >("RenderLookupTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderLookupTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (camera, shader, tag, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
    const CLASS_NAME: &'static str = "SelectionPickerRenderer/SelectionPickerRendererStandard";
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        5usize,
                    >("RenderLookupTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderLookupTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (camera, shader, tag, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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

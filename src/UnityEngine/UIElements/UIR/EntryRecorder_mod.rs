#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryRecorder")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct EntryRecorder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_EntryPool: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::EntryPool>,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryRecorder")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UIR::EntryRecorder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "EntryRecorder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+UIR+EntryRecorder")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::EntryRecorder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+EntryRecorder")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::EntryRecorder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+EntryRecorder")]
impl crate::UnityEngine::UIElements::UIR::EntryRecorder {
    pub fn Append(
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        entry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Append")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Append",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parentEntry, entry))? };
        Ok(__cordl_ret.into())
    }
    pub fn AppendMeshEntry(
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        entry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                    ), quest_hook::libil2cpp::Void, 2usize>("AppendMeshEntry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AppendMeshEntry",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parentEntry, entry))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginStencilMask(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginStencilMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginStencilMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn BlitAndPopRenderTexture(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BlitAndPopRenderTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitAndPopRenderTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn CutRenderChain(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CutRenderChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CutRenderChain", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawChildren(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawChildren")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawChildren", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawGradients(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        vertices: crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        indices: crate::Unity::Collections::NativeSlice_1<u16>,
        gradientsOwner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        crate::Unity::Collections::NativeSlice_1<
                            crate::UnityEngine::UIElements::Vertex,
                        >,
                        crate::Unity::Collections::NativeSlice_1<u16>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawGradients")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawGradients",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (parentEntry, vertices, indices, gradientsOwner))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawImmediate(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cullingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        quest_hook::libil2cpp::Gc<crate::System::Action>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawImmediate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawImmediate",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (parentEntry, callback, cullingEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        vertices: crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        indices: crate::Unity::Collections::NativeSlice_1<u16>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        skipAtlas: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        crate::Unity::Collections::NativeSlice_1<
                            crate::UnityEngine::UIElements::Vertex,
                        >,
                        crate::Unity::Collections::NativeSlice_1<u16>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (parentEntry, vertices, indices, texture, skipAtlas))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRasterText(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        vertices: crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        indices: crate::Unity::Collections::NativeSlice_1<u16>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        multiChannel: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        crate::Unity::Collections::NativeSlice_1<
                            crate::UnityEngine::UIElements::Vertex,
                        >,
                        crate::Unity::Collections::NativeSlice_1<u16>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawRasterText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRasterText",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (parentEntry, vertices, indices, texture, multiChannel),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawSdfText(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        vertices: crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        indices: crate::Unity::Collections::NativeSlice_1<u16>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scale: f32,
        sharpness: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        crate::Unity::Collections::NativeSlice_1<
                            crate::UnityEngine::UIElements::Vertex,
                        >,
                        crate::Unity::Collections::NativeSlice_1<u16>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawSdfText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawSdfText",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (parentEntry, vertices, indices, texture, scale, sharpness),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndStencilMask(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndStencilMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndStencilMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn InsertPlaceholder(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >,
                        1usize,
                    >("InsertPlaceholder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertPlaceholder", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry> =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        entryPool: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::EntryPool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entryPool))?;
        Ok(__cordl_object.into())
    }
    pub fn PopClippingRect(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PopClippingRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopClippingRect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopDefaultMaterial(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PopDefaultMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopDefaultMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopGroupMatrix(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PopGroupMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopGroupMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopScissors(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PopScissors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopScissors", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopStencilMask(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PopStencilMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopStencilMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PushClippingRect(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PushClippingRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PushClippingRect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PushDefaultMaterial(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "PushDefaultMaterial"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PushDefaultMaterial",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn PushGroupMatrix(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PushGroupMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PushGroupMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PushRenderTexture(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PushRenderTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PushRenderTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PushScissors(
        &mut self,
        parentEntry: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Entry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Entry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PushScissors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PushScissors", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        entryPool: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::EntryPool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::EntryPool,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (entryPool))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+EntryRecorder")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIR::EntryRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

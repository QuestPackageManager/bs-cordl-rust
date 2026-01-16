#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScriptableRenderContext {
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::ScriptableRenderContext {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ScriptableRenderContext";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl crate::UnityEngine::Rendering::ScriptableRenderContext {
    #[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext")]
    pub type CullShadowCastersContext =
        crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext;
    pub fn BeginRenderPass(
        &mut self,
        width: i32,
        height: i32,
        samples: i32,
        attachments: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::AttachmentDescriptor,
        >,
        depthAttachmentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::AttachmentDescriptor,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BeginRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRenderPass",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (width, height, samples, attachments, depthAttachmentIndex),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginRenderPass_Internal(
        _cordl_self: crate::System::IntPtr,
        width: i32,
        height: i32,
        volumeDepth: i32,
        samples: i32,
        colors: crate::System::IntPtr,
        colorCount: i32,
        depthAttachmentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::System::IntPtr,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BeginRenderPass_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRenderPass_Internal",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _cordl_self,
                    width,
                    height,
                    volumeDepth,
                    samples,
                    colors,
                    colorCount,
                    depthAttachmentIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSubPass_Internal(
        _cordl_self: crate::System::IntPtr,
        colors: crate::System::IntPtr,
        colorCount: i32,
        inputs: crate::System::IntPtr,
        inputCount: i32,
        isDepthReadOnly: bool,
        isStencilReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                        i32,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "BeginSubPass_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginSubPass_Internal",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _cordl_self,
                    colors,
                    colorCount,
                    inputs,
                    inputCount,
                    isDepthReadOnly,
                    isStencilReadOnly,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSubPass_NativeArray_1__cordl_bool0(
        &mut self,
        colors: crate::Unity::Collections::NativeArray_1<i32>,
        inputs: crate::Unity::Collections::NativeArray_1<i32>,
        isDepthStencilReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<i32>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("BeginSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginSubPass",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (colors, inputs, isDepthStencilReadOnly))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSubPass__cordl_bool1(
        &mut self,
        colors: crate::Unity::Collections::NativeArray_1<i32>,
        isDepthStencilReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BeginSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSubPass", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colors, isDepthStencilReadOnly))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGizmoRendererList(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        gizmoSubset: crate::UnityEngine::Rendering::GizmoSubset,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::GizmoSubset,
                    ), crate::UnityEngine::Rendering::RendererList, 2usize>(
                        "CreateGizmoRendererList",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGizmoRendererList",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, gizmoSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGizmoRendererList_Internal(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        gizmoSubset: crate::UnityEngine::Rendering::GizmoSubset,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::GizmoSubset,
                    ), crate::UnityEngine::Rendering::RendererList, 2usize>(
                        "CreateGizmoRendererList_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGizmoRendererList_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, gizmoSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGizmoRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
        gizmoSubset: crate::UnityEngine::Rendering::GizmoSubset,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::GizmoSubset,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CreateGizmoRendererList_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGizmoRendererList_Internal_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, camera, gizmoSubset, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererList(
        &mut self,
        param: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererListParams>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RendererListParams,
                    >), crate::UnityEngine::Rendering::RendererList, 1usize>(
                        "CreateRendererList"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererList",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (param))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererList_Internal(
        &mut self,
        cullResults: crate::System::IntPtr,
        drawingSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::DrawingSettings,
        >,
        filteringSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::FilteringSettings,
        >,
        tagName: crate::UnityEngine::Rendering::ShaderTagId,
        isPassTagName: bool,
        tagValues: crate::System::IntPtr,
        stateBlocks: crate::System::IntPtr,
        stateCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::DrawingSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::FilteringSettings,
                        >,
                        crate::UnityEngine::Rendering::ShaderTagId,
                        bool,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                    ), crate::UnityEngine::Rendering::RendererList, 8usize>(
                        "CreateRendererList_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererList_Internal",
                            8usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cullResults,
                    drawingSettings,
                    filteringSettings,
                    tagName,
                    isPassTagName,
                    tagValues,
                    stateBlocks,
                    stateCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        cullResults: crate::System::IntPtr,
        drawingSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::DrawingSettings,
        >,
        filteringSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::FilteringSettings,
        >,
        tagName: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::ShaderTagId>,
        isPassTagName: bool,
        tagValues: crate::System::IntPtr,
        stateBlocks: crate::System::IntPtr,
        stateCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::DrawingSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::FilteringSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::ShaderTagId>,
                        bool,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "CreateRendererList_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererList_Internal_Injected",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    cullResults,
                    drawingSettings,
                    filteringSettings,
                    tagName,
                    isPassTagName,
                    tagValues,
                    stateBlocks,
                    stateCount,
                    ret,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShadowRendererList(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowDrawingSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::ShadowDrawingSettings,
                    >), crate::UnityEngine::Rendering::RendererList, 1usize>(
                        "CreateShadowRendererList",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateShadowRendererList",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (settings))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShadowRendererList_Internal(
        &mut self,
        shadowDrawinSettings: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        crate::UnityEngine::Rendering::RendererList,
                        1usize,
                    >("CreateShadowRendererList_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateShadowRendererList_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (shadowDrawinSettings))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateShadowRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        shadowDrawinSettings: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "CreateShadowRendererList_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateShadowRendererList_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, shadowDrawinSettings, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Camera2(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::Rendering::RendererList,
                        1usize,
                    >("CreateSkyboxRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSkyboxRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Internal(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        mode: i32,
        proj: crate::UnityEngine::Matrix4x4,
        view: crate::UnityEngine::Matrix4x4,
        projR: crate::UnityEngine::Matrix4x4,
        viewR: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                    ), crate::UnityEngine::Rendering::RendererList, 6usize>(
                        "CreateSkyboxRendererList_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateSkyboxRendererList_Internal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera, mode, proj, view, projR, viewR))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
        mode: i32,
        proj: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        view: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        projR: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewR: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "CreateSkyboxRendererList_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateSkyboxRendererList_Internal_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_unity_self, camera, mode, proj, view, projR, viewR, ret),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Matrix4x4_Matrix4x4_1(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        viewMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                    ), crate::UnityEngine::Rendering::RendererList, 3usize>(
                        "CreateSkyboxRendererList",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateSkyboxRendererList",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera, projectionMatrix, viewMatrix))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSkyboxRendererList_Matrix4x4_Matrix4x4_Matrix4x4_Matrix4x4_0(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        projectionMatrixL: crate::UnityEngine::Matrix4x4,
        viewMatrixL: crate::UnityEngine::Matrix4x4,
        projectionMatrixR: crate::UnityEngine::Matrix4x4,
        viewMatrixR: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                    ), crate::UnityEngine::Rendering::RendererList, 5usize>(
                        "CreateSkyboxRendererList",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateSkyboxRendererList",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    camera,
                    projectionMatrixL,
                    viewMatrixL,
                    projectionMatrixR,
                    viewMatrixR,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_Camera0(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::Rendering::RendererList,
                        1usize,
                    >("CreateUIOverlayRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateUIOverlayRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_Internal(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        uiSubset: crate::UnityEngine::Rendering::UISubset,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::UISubset,
                    ), crate::UnityEngine::Rendering::RendererList, 2usize>(
                        "CreateUIOverlayRendererList_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateUIOverlayRendererList_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, uiSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
        uiSubset: crate::UnityEngine::Rendering::UISubset,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::UISubset,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CreateUIOverlayRendererList_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateUIOverlayRendererList_Internal_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, camera, uiSubset, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateUIOverlayRendererList_UISubset1(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        uiSubset: crate::UnityEngine::Rendering::UISubset,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::UISubset,
                    ), crate::UnityEngine::Rendering::RendererList, 2usize>(
                        "CreateUIOverlayRendererList",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateUIOverlayRendererList",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, uiSubset))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWireOverlayRendererList(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::Rendering::RendererList,
                        1usize,
                    >("CreateWireOverlayRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateWireOverlayRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWireOverlayRendererList_Internal(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererList> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::Rendering::RendererList,
                        1usize,
                    >("CreateWireOverlayRendererList_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateWireOverlayRendererList_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererList =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateWireOverlayRendererList_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "CreateWireOverlayRendererList_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateWireOverlayRendererList_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, camera, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn Cull(
        &mut self,
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CullingResults> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::ScriptableCullingParameters,
                    >), crate::UnityEngine::Rendering::CullingResults, 1usize>(
                        "Cull"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cull",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::CullingResults =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn CullShadowCasters(
        &mut self,
        cullingResults: crate::UnityEngine::Rendering::CullingResults,
        infos: crate::UnityEngine::Rendering::ShadowCastersCullingInfos,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::CullingResults,
                        crate::UnityEngine::Rendering::ShadowCastersCullingInfos,
                    ), quest_hook::libil2cpp::Void, 2usize>("CullShadowCasters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CullShadowCasters",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cullingResults, infos))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireOverlay(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawWireOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawWireOverlay", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireOverlay_Impl(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawWireOverlay_Impl")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawWireOverlay_Impl", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawWireOverlay_Impl_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DrawWireOverlay_Impl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawWireOverlay_Impl_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn EmitGeometryForCamera(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EmitGeometryForCamera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EmitGeometryForCamera", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn EmitGeometryForCamera_Injected(
        camera: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EmitGeometryForCamera_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EmitGeometryForCamera_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndRenderPass(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EndRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndRenderPass",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EndRenderPass_Internal(
        _cordl_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndRenderPass_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndRenderPass_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSubPass(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EndSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndSubPass",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSubPass_Internal(
        _cordl_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSubPass_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSubPass_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ScriptableRenderContext0(
        &mut self,
        other: crate::UnityEngine::Rendering::ScriptableRenderContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::ScriptableRenderContext),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBuffer(
        &mut self,
        commandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExecuteCommandBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteCommandBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (commandBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBufferAsync(
        &mut self,
        commandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::ComputeQueueType,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ExecuteCommandBufferAsync"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteCommandBufferAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (commandBuffer, queueType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBufferAsync_Internal(
        &mut self,
        commandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::ComputeQueueType,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ExecuteCommandBufferAsync_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteCommandBufferAsync_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (commandBuffer, queueType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBufferAsync_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        commandBuffer: crate::System::IntPtr,
        queueType: crate::UnityEngine::Rendering::ComputeQueueType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::ComputeQueueType,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ExecuteCommandBufferAsync_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteCommandBufferAsync_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, commandBuffer, queueType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBuffer_Internal(
        &mut self,
        commandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ExecuteCommandBuffer_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteCommandBuffer_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (commandBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBuffer_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        commandBuffer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ExecuteCommandBuffer_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExecuteCommandBuffer_Internal_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, commandBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameras(
        &mut self,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("GetCameras")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameras",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (results))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameras_Internal(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resultList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GetCameras_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameras_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (listType, resultList))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasInvokeOnRenderObjectCallbacks(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("HasInvokeOnRenderObjectCallbacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasInvokeOnRenderObjectCallbacks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasInvokeOnRenderObjectCallbacks_Internal() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>(
                        "HasInvokeOnRenderObjectCallbacks_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasInvokeOnRenderObjectCallbacks_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSortSettings(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        sortingSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SortingSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SortingSettings,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InitializeSortSettings"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitializeSortSettings",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera, sortingSettings))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeSortSettings_Injected(
        camera: crate::System::IntPtr,
        sortingSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SortingSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SortingSettings,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InitializeSortSettings_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitializeSortSettings_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (camera, sortingSettings))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Cull(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
        renderLoop: crate::UnityEngine::Rendering::ScriptableRenderContext,
        results: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableCullingParameters,
                        >,
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 3usize>("Internal_Cull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_Cull",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parameters, renderLoop, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CullShadowCasters(
        renderLoop: crate::UnityEngine::Rendering::ScriptableRenderContext,
        context: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_CullShadowCasters"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_CullShadowCasters",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderLoop, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CullShadowCasters_Injected(
        renderLoop: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        context: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_CullShadowCasters_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_CullShadowCasters_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderLoop, context))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Cull_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableCullingParameters,
        >,
        renderLoop: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        results: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableCullingParameters,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_Cull_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_Cull_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (parameters, renderLoop, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetPtr(&mut self) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::IntPtr, 0usize>("Internal_GetPtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetPtr",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareRendererListsAsync(
        &mut self,
        rendererLists: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::RendererList,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "PrepareRendererListsAsync"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareRendererListsAsync",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererLists))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareRendererListsAsync_Internal(
        &mut self,
        rendererLists: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PrepareRendererListsAsync_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareRendererListsAsync_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererLists))? };
        Ok(__cordl_ret.into())
    }
    pub fn QueryRendererListStatus(
        &mut self,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererListStatus> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RendererList),
                        crate::UnityEngine::Rendering::RendererListStatus,
                        1usize,
                    >("QueryRendererListStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "QueryRendererListStatus", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererListStatus =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn QueryRendererListStatus_Internal(
        &mut self,
        handle: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererListStatus> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RendererList),
                        crate::UnityEngine::Rendering::RendererListStatus,
                        1usize,
                    >("QueryRendererListStatus_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "QueryRendererListStatus_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererListStatus =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn QueryRendererListStatus_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        handle: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RendererListStatus> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), crate::UnityEngine::Rendering::RendererListStatus, 2usize>(
                        "QueryRendererListStatus_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "QueryRendererListStatus_Internal_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RendererListStatus =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_Camera__cordl_bool0(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        stereoSetup: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetupCameraProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupCameraProperties", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, stereoSetup))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_Internal(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        stereoSetup: bool,
        eye: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetupCameraProperties_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupCameraProperties_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, stereoSetup, eye))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        camera: crate::System::IntPtr,
        stereoSetup: bool,
        eye: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                        >,
                        crate::System::IntPtr,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetupCameraProperties_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupCameraProperties_Internal_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, camera, stereoSetup, eye))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_i32_1(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        stereoSetup: bool,
        eye: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetupCameraProperties"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupCameraProperties",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, stereoSetup, eye))? };
        Ok(__cordl_ret.into())
    }
    pub fn Submit(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Submit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Submit",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitForRenderPassValidation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("SubmitForRenderPassValidation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SubmitForRenderPassValidation",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitForRenderPassValidation_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("SubmitForRenderPassValidation_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SubmitForRenderPassValidation_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Submit_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Submit_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Submit_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ptr))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::ScriptableRenderContext>>
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::ScriptableRenderContext> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::ScriptableRenderContext>>
    for crate::UnityEngine::Rendering::ScriptableRenderContext
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Rendering::ScriptableRenderContext>
    {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScriptableRenderContext_CullShadowCastersContext {
    pub cullResults: crate::System::IntPtr,
    pub splitBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub splitBufferLength: i32,
    pub perLightInfos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub perLightInfoCount: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ScriptableRenderContext/CullShadowCastersContext";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext+CullShadowCastersContext")]
impl crate::UnityEngine::Rendering::ScriptableRenderContext_CullShadowCastersContext {}

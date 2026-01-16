#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RenderChainVEData {
    pub prev: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub next: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub groupTransformAncestor:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub boneTransformAncestor:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub prevDirty: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub nextDirty: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub flags: crate::UnityEngine::UIElements::UIR::RenderDataFlags,
    pub hierarchyDepth: i32,
    pub dirtiedValues: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
    pub dirtyID: u32,
    pub firstHeadCommand:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
    pub lastHeadCommand:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
    pub firstTailCommand:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
    pub lastTailCommand:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
    pub localFlipsWinding: bool,
    pub localTransformScaleZero: bool,
    pub worldFlipsWinding: bool,
    pub worldTransformScaleZero: bool,
    pub clipMethod: crate::UnityEngine::UIElements::UIR::ClipMethod,
    pub childrenStencilRef: i32,
    pub childrenMaskDepth: i32,
    pub headMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    pub tailMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    pub verticesSpace: crate::UnityEngine::Matrix4x4,
    pub transformID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub clipRectID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub opacityID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub textCoreSettingsID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub colorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub backgroundColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderLeftColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderTopColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderRightColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub borderBottomColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub tintColorID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub compositeOpacity: f32,
    pub backgroundAlpha: f32,
    pub textures: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BasicNode_1<
            crate::UnityEngine::UIElements::UIR::TextureEntry,
        >,
    >,
    pub pendingRepaint: bool,
    pub pendingHierarchicalRepaint: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderChainVEData";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::RenderChainVEData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::RenderChainVEData
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::RenderChainVEData
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::RenderChainVEData
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::RenderChainVEData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
impl crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    pub fn AllocatesID(
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        bool,
                        1usize,
                    >("AllocatesID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocatesID", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (alloc))? };
        Ok(__cordl_ret.into())
    }
    pub fn InheritsID(
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        bool,
                        1usize,
                    >("InheritsID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InheritsID", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (alloc))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasExtraData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasExtraData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hasExtraData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasExtraMeshes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasExtraMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hasExtraMeshes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isGroupTransform(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isGroupTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isGroupTransform",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isIgnoringDynamicColorHint(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isIgnoringDynamicColorHint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isIgnoringDynamicColorHint",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isInChain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isInChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isInChain",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lastTailOrHeadCommand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                    >, 0usize>("get_lastTailOrHeadCommand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_lastTailOrHeadCommand",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}

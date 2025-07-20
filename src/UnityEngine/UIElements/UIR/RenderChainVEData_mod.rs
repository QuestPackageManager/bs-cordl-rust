#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderChainVEData {
    pub prev: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub next: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub groupTransformAncestor: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub boneTransformAncestor: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub prevDirty: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub nextDirty: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub flags: crate::UnityEngine::UIElements::UIR::RenderDataFlags,
    pub hierarchyDepth: i32,
    pub dirtiedValues: crate::UnityEngine::UIElements::UIR::RenderDataDirtyTypes,
    pub dirtyID: u32,
    pub firstCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub lastCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub firstClosingCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub lastClosingCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub isInChain: bool,
    pub isHierarchyHidden: bool,
    pub localFlipsWinding: bool,
    pub localTransformScaleZero: bool,
    pub worldFlipsWinding: bool,
    pub worldTransformScaleZero: bool,
    pub clipMethod: crate::UnityEngine::UIElements::UIR::Implementation::ClipMethod,
    pub childrenStencilRef: i32,
    pub childrenMaskDepth: i32,
    pub disableNudging: bool,
    pub data: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    pub closingData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::MeshHandle,
    >,
    pub verticesSpace: crate::UnityEngine::Matrix4x4,
    pub displacementUVStart: i32,
    pub displacementUVEnd: i32,
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
    pub backgroundColor: crate::UnityEngine::Color,
    pub textures: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BasicNode_1<
            crate::UnityEngine::UIElements::UIR::TextureEntry,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderChainVEData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::RenderChainVEData {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::RenderChainVEData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                bool,
                1usize,
            >("AllocatesID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::RenderChainVEData as
                    quest_hook::libil2cpp::Type > ::class(), "AllocatesID", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (alloc))? };
        Ok(__cordl_ret.into())
    }
    pub fn InheritsID(
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::RenderChainVEData as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                bool,
                1usize,
            >("InheritsID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::RenderChainVEData as
                    quest_hook::libil2cpp::Type > ::class(), "InheritsID", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (alloc))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isIgnoringDynamicColorHint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::RenderChainVEData as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isIgnoringDynamicColorHint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::RenderChainVEData as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_isIgnoringDynamicColorHint", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lastClosingOrLastCommand(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::RenderChainVEData as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                >,
                0usize,
            >("get_lastClosingOrLastCommand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::RenderChainVEData as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_lastClosingOrLastCommand", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}

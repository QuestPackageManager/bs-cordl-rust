#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Copy", derive(Copy))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[cfg_attr(feature = "derive_Eq", derive(Eq))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[repr(i32)]
pub enum VisualElementFlags {
    #[cfg_attr(feature = "derive_Default", default)]
    BoundingBoxDirty = 8i32,
    CompositeRoot = 128i32,
    DetachedDataSource = 131072i32,
    DisableClipping = 1024i32,
    DisableRendering = 16384i32,
    EnableViewDataPersistence = 512i32,
    EventInterestParentCategoriesDirty = 32i32,
    HierarchyDisplayed = 4096i32,
    Init = 196671i32,
    LayoutManual = 64i32,
    LocalBounds3DDirty = 65536i32,
    Needs3DBounds = 32768i32,
    NeedsAttachToPanelEvent = 2048i32,
    RequireMeasureFunction = 256i32,
    StyleInitialized = 8192i32,
    WorldBoundingBoxDirty = 16i32,
    WorldClipDirty = 4i32,
    WorldTransformDirty = 1i32,
    WorldTransformInverseDirty = 2i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::VisualElementFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "VisualElementFlags";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::UIElements::VisualElementFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::VisualElementFlags
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::UIElements::VisualElementFlags {
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+VisualElementFlags")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::UIElements::VisualElementFlags {
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

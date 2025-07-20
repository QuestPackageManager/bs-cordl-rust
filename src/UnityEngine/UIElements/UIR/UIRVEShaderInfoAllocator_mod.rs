#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UIRVEShaderInfoAllocator {
    pub m_Storage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
    >,
    pub m_TransformAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ClipRectAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_OpacityAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ColorAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_TextSettingsAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_StorageReallyCreated: bool,
    pub m_VertexTexturingEnabled: bool,
    pub m_Transforms: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::UIElements::UIR::Transform3x4,
    >,
    pub m_ClipRects: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Vector4,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRVEShaderInfoAllocator";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    pub fn AllocClipRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UIElements::UIR::BMPAlloc,
                0usize,
            >("AllocClipRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocClipRect", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UIElements::UIR::BMPAlloc,
                0usize,
            >("AllocColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocColor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocOpacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UIElements::UIR::BMPAlloc,
                0usize,
            >("AllocOpacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocOpacity", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocTextCoreSettings(
        &mut self,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::TextCoreSettings),
                crate::UnityEngine::UIElements::UIR::BMPAlloc,
                1usize,
            >("AllocTextCoreSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocTextCoreSettings",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            method.invoke_unchecked(self, (settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocToConstantBufferIndex(
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                i32,
                1usize,
            >("AllocToConstantBufferIndex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "AllocToConstantBufferIndex", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (alloc))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllocToTexelCoord(
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
        >,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
                    >,
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                ),
                crate::UnityEngine::Vector2Int,
                2usize,
            >("AllocToTexelCoord")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocToTexelCoord", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int = unsafe {
            method.invoke_unchecked((), (allocator, alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UIElements::UIR::BMPAlloc,
                0usize,
            >("AllocTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AllocTransform", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AtlasRectMatchesPage(
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
        >,
        defAlloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        atlasRect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
                    >,
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                    crate::UnityEngine::RectInt,
                ),
                bool,
                3usize,
            >("AtlasRectMatchesPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "AtlasRectMatchesPage",
                    3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (allocator, defAlloc, atlasRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClipRectAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                crate::UnityEngine::Color32,
                1usize,
            >("ClipRectAllocToVertexData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "ClipRectAllocToVertexData",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ColorAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                crate::UnityEngine::Color32,
                1usize,
            >("ColorAllocToVertexData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "ColorAllocToVertexData",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Construct(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Construct")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "Construct", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeClipRect(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeClipRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "FreeClipRect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeColor(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeColor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "FreeColor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeOpacity(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeOpacity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "FreeOpacity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTextCoreSettings(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeTextCoreSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "FreeTextCoreSettings",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTransform(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FreeTransform")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "FreeTransform", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePendingStorageChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("IssuePendingStorageChanges")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "IssuePendingStorageChanges", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OpacityAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                crate::UnityEngine::Color32,
                1usize,
            >("OpacityAllocToVertexData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "OpacityAllocToVertexData",
                    1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReallyCreateStorage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ReallyCreateStorage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "ReallyCreateStorage",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetClipRectValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        clipRect: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                    crate::UnityEngine::Vector4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetClipRectValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "SetClipRectValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc, clipRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetColorValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        color: crate::UnityEngine::Color,
        isEditorContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                    crate::UnityEngine::Color,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetColorValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "SetColorValue", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc, color, isEditorContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOpacityValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        opacity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetOpacityValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "SetOpacityValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc, opacity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextCoreSettingValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
        isEditorContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                    crate::UnityEngine::UIElements::UIR::TextCoreSettings,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetTextCoreSettingValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "SetTextCoreSettingValue",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc, settings, isEditorContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTransformValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        xform: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::UIR::BMPAlloc,
                    crate::UnityEngine::Matrix4x4,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetTransformValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "SetTransformValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (alloc, xform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TextCoreSettingsToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                crate::UnityEngine::Color32,
                1usize,
            >("TextCoreSettingsToVertexData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TextCoreSettingsToVertexData", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                crate::UnityEngine::Color32,
                1usize,
            >("TransformAllocToVertexData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TransformAllocToVertexData", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            method.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                0usize,
            >("get_atlas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "get_atlas", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_clipRectConstants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::Vector4>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::Vector4>,
                0usize,
            >("get_clipRectConstants")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "get_clipRectConstants",
                    0usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::Vector4,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pageHeight() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_pageHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "get_pageHeight", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pageWidth() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_pageWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "get_pageWidth", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_transformConstants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::Transform3x4,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Unity::Collections::NativeSlice_1<
                    crate::UnityEngine::UIElements::UIR::Transform3x4,
                >,
                0usize,
            >("get_transformConstants")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator as
                    quest_hook::libil2cpp::Type > ::class(), "get_transformConstants",
                    0usize
                )
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::Transform3x4,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}

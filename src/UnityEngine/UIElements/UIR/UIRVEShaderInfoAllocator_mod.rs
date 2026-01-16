#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRVEShaderInfoAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Storage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
    >,
    pub m_TransformAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ClipRectAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_OpacityAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ColorAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_TextSettingsAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_StorageReallyCreated: bool,
    pub m_ColorSpace: crate::UnityEngine::ColorSpace,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRVEShaderInfoAllocator";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    pub fn AcquireDefaultShaderInfoTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("AcquireDefaultShaderInfoTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AcquireDefaultShaderInfoTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocClipRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::UIR::BMPAlloc,
                        0usize,
                    >("AllocClipRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocClipRect", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::UIR::BMPAlloc,
                        0usize,
                    >("AllocColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocColor", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocOpacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::UIR::BMPAlloc,
                        0usize,
                    >("AllocOpacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocOpacity", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocTextCoreSettings(
        &mut self,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::TextCoreSettings),
                        crate::UnityEngine::UIElements::UIR::BMPAlloc,
                        1usize,
                    >("AllocTextCoreSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocTextCoreSettings", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            cordl_method_info.invoke_unchecked(self, (settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocToTexelCoord(
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
        >,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocToTexelCoord", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int = unsafe {
            cordl_method_info.invoke_unchecked((), (allocator, alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::UIR::BMPAlloc,
                        0usize,
                    >("AllocTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocTransform", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AtlasRectMatchesPage", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (allocator, defAlloc, atlasRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClipRectAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("ClipRectAllocToVertexData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClipRectAllocToVertexData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ColorAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("ColorAllocToVertexData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ColorAllocToVertexData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeClipRect(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeClipRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeClipRect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeColor(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeOpacity(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeOpacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeOpacity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTextCoreSettings(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeTextCoreSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeTextCoreSettings", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeTransform(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeTransform")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeTransform", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePendingStorageChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("IssuePendingStorageChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePendingStorageChanges", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        colorSpace: crate::UnityEngine::ColorSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorSpace))?;
        Ok(__cordl_object.into())
    }
    pub fn OpacityAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("OpacityAllocToVertexData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpacityAllocToVertexData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReallyCreateStorage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ReallyCreateStorage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReallyCreateStorage", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseDefaultShaderInfoTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ReleaseDefaultShaderInfoTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseDefaultShaderInfoTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetClipRectValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        clipRect: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetClipRectValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc, clipRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetColorValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::UIR::BMPAlloc,
                            crate::UnityEngine::Color,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetColorValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetColorValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc, color))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetOpacityValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        opacity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc, f32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetOpacityValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetOpacityValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc, opacity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextCoreSettingValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::UIElements::UIR::BMPAlloc,
                            crate::UnityEngine::UIElements::UIR::TextCoreSettings,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetTextCoreSettingValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetTextCoreSettingValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc, settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTransformValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        xform: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetTransformValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc, xform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TextCoreSettingsToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("TextCoreSettingsToVertexData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TextCoreSettingsToVertexData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::UIR::BMPAlloc),
                        crate::UnityEngine::Color32,
                        1usize,
                    >("TransformAllocToVertexData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformAllocToVertexData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (alloc))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        colorSpace: crate::UnityEngine::ColorSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::ColorSpace),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (colorSpace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        0usize,
                    >("get_atlas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_atlas", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pageHeight() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_pageHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pageHeight", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_pageWidth() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_pageWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_pageWidth", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

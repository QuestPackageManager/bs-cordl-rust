#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NativePassData {
    pub loadAudit: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::FixedAttachmentArray_1<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::LoadAudit,
    >,
    pub storeAudit: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::FixedAttachmentArray_1<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::StoreAudit,
    >,
    pub breakAudit: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
    pub fragments: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::FixedAttachmentArray_1<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
    >,
    pub attachments: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::FixedAttachmentArray_1<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassAttachment,
    >,
    pub firstGraphPass: i32,
    pub lastGraphPass: i32,
    pub numGraphPasses: i32,
    pub firstNativeSubPass: i32,
    pub numNativeSubPasses: i32,
    pub width: i32,
    pub height: i32,
    pub volumeDepth: i32,
    pub samples: i32,
    pub hasDepth: bool,
    pub hasFoveatedRasterization: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "NativePassData";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData {
    pub fn CanMerge(
        contextData: Blacklisted,
        activeNativePassId: i32,
        passIdToMerge: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (Blacklisted, i32, i32),
                        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
                        3usize,
                    >("CanMerge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanMerge", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit = unsafe {
            cordl_method_info
                .invoke_unchecked((), (contextData, activeNativePassId, passIdToMerge))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanMergeNativeSubPass(
        contextData: Blacklisted,
        nativePass: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        passToMerge: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                        ),
                        bool,
                        3usize,
                    >("CanMergeNativeSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanMergeNativeSubPass", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (contextData, nativePass, passToMerge))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphPassNames(
        &mut self,
        ctx: Blacklisted,
        dest: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DynamicArray_1<
                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DynamicArray_1<
                                    crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetGraphPassNames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphPassNames", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx, dest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GraphPasses(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                        >,
                        1usize,
                    >("GraphPasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GraphPasses", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsValid",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPassStatesForNativePass(
        contextData: Blacklisted,
        nativePassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (Blacklisted, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetPassStatesForNativePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPassStatesForNativePass", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (contextData, nativePassId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMerge(
        contextData: Blacklisted,
        activeNativePassId: i32,
        passIdToMerge: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (Blacklisted, i32, i32),
                        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
                        3usize,
                    >("TryMerge")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryMerge", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit = unsafe {
            cordl_method_info
                .invoke_unchecked((), (contextData, activeNativePassId, passIdToMerge))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryMergeNativeSubPass(
        contextData: Blacklisted,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
        passToMerge: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("TryMergeNativeSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryMergeNativeSubPass", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (contextData, nativePass, passToMerge))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateNativeSubPassesAttachments(
        contextData: Blacklisted,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdateNativeSubPassesAttachments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateNativeSubPassesAttachments", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (contextData, nativePass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                            Blacklisted,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (pass, ctx))?
        };
        Ok(__cordl_ret.into())
    }
}

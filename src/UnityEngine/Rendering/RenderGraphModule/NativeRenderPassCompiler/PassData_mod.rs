#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PassData {
    pub passId: i32,
    pub _cordl_type: crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPassType,
    pub hasFoveatedRasterization: bool,
    pub tag: i32,
    pub mergeState:
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassMergeState,
    pub nativePassIndex: i32,
    pub nativeSubPassIndex: i32,
    pub firstInput: i32,
    pub numInputs: i32,
    pub firstOutput: i32,
    pub numOutputs: i32,
    pub firstFragment: i32,
    pub numFragments: i32,
    pub firstFragmentInput: i32,
    pub numFragmentInputs: i32,
    pub firstRandomAccessResource: i32,
    pub numRandomAccessResources: i32,
    pub firstCreate: i32,
    pub numCreated: i32,
    pub firstDestroy: i32,
    pub numDestroyed: i32,
    pub fragmentInfoWidth: i32,
    pub fragmentInfoHeight: i32,
    pub fragmentInfoVolumeDepth: i32,
    pub fragmentInfoSamples: i32,
    pub waitOnGraphicsFencePassId: i32,
    pub asyncCompute: bool,
    pub hasSideEffects: bool,
    pub culled: bool,
    pub beginNativeSubpass: bool,
    pub fragmentInfoValid: bool,
    pub fragmentInfoHasDepth: bool,
    pub insertGraphicsFence: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str =
        "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "PassData";
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+PassData")]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData {
    pub fn AddFirstUse(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddFirstUse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddFirstUse",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddFragment(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddFragment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddFragment",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddFragmentInput(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddFragmentInput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddFragmentInput",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddLastUse(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), quest_hook::libil2cpp::Void, 2usize>("AddLastUse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddLastUse",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddRandomAccessResource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "AddRandomAccessResource",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddRandomAccessResource",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FirstUsedResources(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(Blacklisted), crate::System::ReadOnlySpan_1<
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                    >, 1usize>("FirstUsedResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FirstUsedResources",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn FragmentInputs(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
                        >,
                        1usize,
                    >("FragmentInputs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FragmentInputs", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn Fragments(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
                        >,
                        1usize,
                    >("Fragments")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Fragments", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassFragmentData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name,
                        1usize,
                    >("GetName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GetName",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Inputs(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassInputData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassInputData,
                        >,
                        1usize,
                    >("Inputs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Inputs",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassInputData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsUsedAsFragment(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), bool, 2usize>("IsUsedAsFragment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsUsedAsFragment",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn LastUsedResources(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(Blacklisted), crate::System::ReadOnlySpan_1<
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                    >, 1usize>("LastUsedResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LastUsedResources",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn Outputs(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassOutputData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassOutputData,
                        >,
                        1usize,
                    >("Outputs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Outputs",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassOutputData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn RandomWriteTextures(
        &mut self,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassRandomWriteData,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (Blacklisted),
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassRandomWriteData,
                        >,
                        1usize,
                    >("RandomWriteTextures")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RandomWriteTextures", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassRandomWriteData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAndInitialize(
        &mut self,
        pass: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
            >,
        >,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("ResetAndInitialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetAndInitialize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass, passIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupAndValidateFragmentInfo(
        &mut self,
        h: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        ctx: Blacklisted,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                        Blacklisted,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupAndValidateFragmentInfo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupAndValidateFragmentInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (h, ctx))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pass: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
            >,
        >,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pass, passIndex))? };
        Ok(__cordl_ret.into())
    }
}

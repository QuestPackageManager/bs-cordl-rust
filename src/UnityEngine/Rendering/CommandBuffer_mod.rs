#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CommandBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl crate::UnityEngine::Rendering::CommandBuffer {
    #[cfg(feature = "UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
    pub type BindingsMarshaller = crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller;
    pub fn BeginRenderPass(
        &mut self,
        width: i32,
        height: i32,
        volumeDepth: i32,
        samples: i32,
        attachments: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::AttachmentDescriptor,
        >,
        depthAttachmentIndex: i32,
        subPasses: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::SubPassDescriptor,
        >,
        debugNameUtf8: crate::System::ReadOnlySpan_1<u8>,
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
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::AttachmentDescriptor,
                        >,
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::SubPassDescriptor,
                        >,
                        crate::System::ReadOnlySpan_1<u8>,
                    ), quest_hook::libil2cpp::Void, 8usize>("BeginRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRenderPass",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    width,
                    height,
                    volumeDepth,
                    samples,
                    attachments,
                    depthAttachmentIndex,
                    subPasses,
                    debugNameUtf8,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginRenderPass_Internal(
        &mut self,
        width: i32,
        height: i32,
        volumeDepth: i32,
        samples: i32,
        attachments: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::AttachmentDescriptor,
        >,
        depthAttachmentIndex: i32,
        subPasses: crate::System::ReadOnlySpan_1<crate::UnityEngine::Rendering::SubPassDescriptor>,
        debugNameUtf8: crate::System::ReadOnlySpan_1<u8>,
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
                        i32,
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::AttachmentDescriptor,
                        >,
                        i32,
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::SubPassDescriptor,
                        >,
                        crate::System::ReadOnlySpan_1<u8>,
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
                self,
                (
                    width,
                    height,
                    volumeDepth,
                    samples,
                    attachments,
                    depthAttachmentIndex,
                    subPasses,
                    debugNameUtf8,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginRenderPass_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        width: i32,
        height: i32,
        volumeDepth: i32,
        samples: i32,
        attachments: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        depthAttachmentIndex: i32,
        subPasses: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        debugNameUtf8: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "BeginRenderPass_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginRenderPass_Internal_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    width,
                    height,
                    volumeDepth,
                    samples,
                    attachments,
                    depthAttachmentIndex,
                    subPasses,
                    debugNameUtf8,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_CustomSampler(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample_CustomSampler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample_CustomSampler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_CustomSampler1(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_CustomSampler_Injected(
        _unity_self: crate::System::IntPtr,
        sampler: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BeginSample_CustomSampler_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample_CustomSampler_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_Injected(
        _unity_self: crate::System::IntPtr,
        name: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "BeginSample_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BeginSample_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Identifier(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector2,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>("Blit_Identifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit_Identifier",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Identifier_Injected(
        _unity_self: crate::System::IntPtr,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: crate::System::IntPtr,
        pass: i32,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Blit_Identifier_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit_Identifier_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RenderTargetIdentifier2(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RenderTargetIdentifier_Material3(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, dest, mat))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RenderTargetIdentifier_Material_i32_4(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, dest, mat, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Texture(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector2,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>("Blit_Texture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit_Texture",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Texture0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Texture_Injected(
        _unity_self: crate::System::IntPtr,
        source: crate::System::IntPtr,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: crate::System::IntPtr,
        pass: i32,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Blit_Texture_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit_Texture_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Texture_Material1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, dest, mat))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRayTracingAccelerationStructure_RayTracingAccelerationStructure0(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "BuildRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildRayTracingAccelerationStructure",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accelerationStructure))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRayTracingAccelerationStructure_Vector3_1(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
        relativeOrigin: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "BuildRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildRayTracingAccelerationStructure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (accelerationStructure, relativeOrigin))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckThrowOnSetRenderTarget(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "CheckThrowOnSetRenderTarget",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckThrowOnSetRenderTarget",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRandomWriteTargets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ClearRandomWriteTargets",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRandomWriteTargets",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRandomWriteTargets_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("ClearRandomWriteTargets_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRandomWriteTargets_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTargetMulti_Internal(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                        >,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ClearRenderTargetMulti_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTargetMulti_Internal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (clearFlags, colors, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTargetMulti_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        colors: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::RTClearFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "ClearRenderTargetMulti_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTargetMulti_Internal_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, clearFlags, colors, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTargetSingle_Internal(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        color: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        crate::UnityEngine::Color,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ClearRenderTargetSingle_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTargetSingle_Internal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (clearFlags, color, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTargetSingle_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::RTClearFlags,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "ClearRenderTargetSingle_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTargetSingle_Internal_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, clearFlags, color, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget_RTClearFlags_Color_f32_u32_3(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        crate::UnityEngine::Color,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearFlags, backgroundColor, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget_RTClearFlags_Il2CppArray_f32_u32_4(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        backgroundColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                        >,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearFlags, backgroundColors, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color0(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (clearDepth, clearColor, backgroundColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color_f32_1(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearDepth, clearColor, backgroundColor, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color_f32_u32_2(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color, f32, u32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (clearDepth, clearColor, backgroundColor, depth, stencil),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("Clear_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Clear_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureFoveatedRendering(
        &mut self,
        platformData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "ConfigureFoveatedRendering",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConfigureFoveatedRendering",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (platformData))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureFoveatedRendering_Injected(
        _unity_self: crate::System::IntPtr,
        platformData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ConfigureFoveatedRendering_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureFoveatedRendering_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, platformData))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueCC(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValueCC")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueCC",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueCC_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        dst: crate::System::IntPtr,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CopyCounterValueCC_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueCC_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueCG(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValueCG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueCG",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueCG_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        dst: crate::System::IntPtr,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CopyCounterValueCG_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueCG_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueGC(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValueGC")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueGC",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueGC_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        dst: crate::System::IntPtr,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CopyCounterValueGC_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueGC_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueGG(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValueGG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueGG",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValueGG_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        dst: crate::System::IntPtr,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CopyCounterValueGG_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValueGG_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_ComputeBuffer0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_GraphicsBuffer2(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_ComputeBuffer1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_GraphicsBuffer3(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Internal(
        &mut self,
        src: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        srcElement: i32,
        srcMip: i32,
        srcX: i32,
        srcY: i32,
        srcWidth: i32,
        srcHeight: i32,
        dst: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        dstElement: i32,
        dstMip: i32,
        dstX: i32,
        dstY: i32,
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 13usize>(
                        "CopyTexture_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture_Internal",
                            13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    src, srcElement, srcMip, srcX, srcY, srcWidth, srcHeight, dst, dstElement,
                    dstMip, dstX, dstY, mode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        src: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        srcElement: i32,
        srcMip: i32,
        srcX: i32,
        srcY: i32,
        srcWidth: i32,
        srcHeight: i32,
        dst: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        dstElement: i32,
        dstMip: i32,
        dstX: i32,
        dstY: i32,
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 14usize>(
                        "CopyTexture_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture_Internal_Injected",
                            14usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    srcElement,
                    srcMip,
                    srcX,
                    srcY,
                    srcWidth,
                    srcHeight,
                    dst,
                    dstElement,
                    dstMip,
                    dstX,
                    dstY,
                    mode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_RenderTargetIdentifier0(
        &mut self,
        src: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dst: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_i32_i32_RenderTargetIdentifier_i32_i32_1(
        &mut self,
        src: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        srcElement: i32,
        srcMip: i32,
        dst: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dstElement: i32,
        dstMip: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (src, srcElement, srcMip, dst, dstElement, dstMip))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_i32_i32_i32_i32_i32_i32_RenderTargetIdentifier_i32_i32_i32_i32_2(
        &mut self,
        src: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        srcElement: i32,
        srcMip: i32,
        srcX: i32,
        srcY: i32,
        srcWidth: i32,
        srcHeight: i32,
        dst: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dstElement: i32,
        dstMip: i32,
        dstX: i32,
        dstY: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 12usize>("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    src, srcElement, srcMip, srcX, srcY, srcWidth, srcHeight, dst, dstElement,
                    dstMip, dstX, dstY,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAsyncGraphicsFence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::GraphicsFence> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Rendering::GraphicsFence, 0usize>(
                        "CreateAsyncGraphicsFence",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateAsyncGraphicsFence",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsFence =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGPUFence_Internal(
        &mut self,
        fenceType: crate::UnityEngine::Rendering::GraphicsFenceType,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::GraphicsFenceType,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), crate::System::IntPtr, 2usize>("CreateGPUFence_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGPUFence_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked(self, (fenceType, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGPUFence_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        fenceType: crate::UnityEngine::Rendering::GraphicsFenceType,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::GraphicsFenceType,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), crate::System::IntPtr, 3usize>(
                        "CreateGPUFence_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGPUFence_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, fenceType, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGraphicsFence(
        &mut self,
        fenceType: crate::UnityEngine::Rendering::GraphicsFenceType,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::GraphicsFence> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::GraphicsFenceType,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), crate::UnityEngine::Rendering::GraphicsFence, 2usize>(
                        "CreateGraphicsFence"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateGraphicsFence",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsFence =
            unsafe { cordl_method_info.invoke_unchecked(self, (fenceType, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableComputeKeyword(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DisableComputeKeyword"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableComputeKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableComputeKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "DisableComputeKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableComputeKeyword_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableGlobalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::GlobalKeyword),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisableGlobalKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableGlobalKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableGlobalKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DisableGlobalKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableGlobalKeyword_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableMaterialKeyword(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DisableMaterialKeyword"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableMaterialKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableMaterialKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        material: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "DisableMaterialKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableMaterialKeyword_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableScissorRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DisableScissorRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableScissorRect",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableScissorRect_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("DisableScissorRect_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableScissorRect_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableShaderKeyword(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisableShaderKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableShaderKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableShaderKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DisableShaderKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableShaderKeyword_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_ComputeBuffer_u32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_GraphicsBuffer_u32_2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_i32_i32_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        threadGroupsX: i32,
        threadGroupsY: i32,
        threadGroupsZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    threadGroupsX,
                    threadGroupsY,
                    threadGroupsZ,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchRays(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        rayGenName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: u32,
        height: u32,
        depth: u32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                        u32,
                        u32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), quest_hook::libil2cpp::Void, 6usize>("DispatchRays")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchRays",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, rayGenName, width, height, depth, camera),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disposing))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer5(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer_i32_4(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer_i32_MaterialPropertyBlock3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedProcedural(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedProcedural"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, count, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Mesh_i32_Material_i32_Il2CppArray2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (mesh, submeshIndex, material, shaderPass, matrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, matrices, count),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    matrices,
                    count,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_Mesh_Matrix4x4_Material3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mesh, matrix, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (mesh, matrix, material, submeshIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
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
                .invoke_unchecked(self, (mesh, matrix, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMultipleMeshes(
        &mut self,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>>,
        >,
        subsetIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        count: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawMultipleMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMultipleMeshes",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrices,
                    meshes,
                    subsetIndices,
                    count,
                    material,
                    shaderPass,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawOcclusionMesh(
        &mut self,
        normalizedCamViewport: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::RectInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawOcclusionMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawOcclusionMesh", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (normalizedCamViewport))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer5(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_4(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_MaterialPropertyBlock3(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer11(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_10(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_MaterialPropertyBlock9(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer2(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (matrix, material, shaderPass, topology, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_1(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_MaterialPropertyBlock0(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer8(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (matrix, material, shaderPass, topology, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_7(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_MaterialPropertyBlock6(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_5(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_i32_4(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_i32_MaterialPropertyBlock3(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_2(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (matrix, material, shaderPass, topology, vertexCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_i32_1(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_i32_MaterialPropertyBlock0(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRendererList(
        &mut self,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RendererList),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_Renderer_Material2(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderer, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_i32_1(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderer, material, submeshIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_i32_i32_0(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableComputeKeyword(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableComputeKeyword"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableComputeKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableComputeKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "EnableComputeKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableComputeKeyword_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, keyword))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableGlobalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::GlobalKeyword),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableGlobalKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableGlobalKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableGlobalKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableGlobalKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableGlobalKeyword_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableMaterialKeyword(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableMaterialKeyword"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableMaterialKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableMaterialKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        material: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "EnableMaterialKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableMaterialKeyword_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableScissorRect(
        &mut self,
        scissor: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rect), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableScissorRect",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableScissorRect",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scissor))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableScissorRect_Injected(
        _unity_self: crate::System::IntPtr,
        scissor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableScissorRect_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableScissorRect_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, scissor))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableShaderKeyword(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableShaderKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableShaderKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableShaderKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "EnableShaderKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableShaderKeyword_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, keyword))? };
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
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "EndRenderPass_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndRenderPass_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EndRenderPass_Internal_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("EndRenderPass_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndRenderPass_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_CustomSampler(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample_CustomSampler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample_CustomSampler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_CustomSampler1(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_CustomSampler_Injected(
        _unity_self: crate::System::IntPtr,
        sampler: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EndSample_CustomSampler_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample_CustomSampler_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_Injected(
        _unity_self: crate::System::IntPtr,
        name: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("EndSample_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EndSample_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Finalize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRTWithDescriptor(
        &mut self,
        nameID: i32,
        desc: crate::UnityEngine::RenderTextureDescriptor,
        filter: crate::UnityEngine::FilterMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::RenderTextureDescriptor,
                        crate::UnityEngine::FilterMode,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "GetTemporaryRTWithDescriptor"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRTWithDescriptor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, desc, filter))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRTWithDescriptor_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        desc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filter: crate::UnityEngine::FilterMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "GetTemporaryRTWithDescriptor_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRTWithDescriptor_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, desc, filter))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        width: i32,
        height: i32,
        filter: crate::UnityEngine::FilterMode,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
        useDynamicScale: bool,
        shadowSamplingMode: crate::UnityEngine::Rendering::ShadowSamplingMode,
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
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        bool,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 12usize>(
                        "GetTemporaryRT_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT_Injected",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    nameID,
                    width,
                    height,
                    filter,
                    colorFormat,
                    depthStencilFormat,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                    useDynamicScale,
                    shadowSamplingMode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_RenderTextureDescriptor_FilterMode7(
        &mut self,
        nameID: i32,
        desc: crate::UnityEngine::RenderTextureDescriptor,
        filter: crate::UnityEngine::FilterMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::RenderTextureDescriptor,
                        crate::UnityEngine::FilterMode,
                    ), quest_hook::libil2cpp::Void, 3usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, desc, filter))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_FilterMode_GraphicsFormat_GraphicsFormat_i32__cordl_bool_RenderTextureMemoryless__cordl_bool_ShadowSamplingMode0(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        filter: crate::UnityEngine::FilterMode,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
        useDynamicScale: bool,
        shadowSamplingMode: crate::UnityEngine::Rendering::ShadowSamplingMode,
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
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        bool,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 11usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    filter,
                    colorFormat,
                    depthStencilFormat,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                    useDynamicScale,
                    shadowSamplingMode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat6(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                    ), quest_hook::libil2cpp::Void, 6usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (nameID, width, height, depthBuffer, filter, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat_RenderTextureReadWrite5(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                    ), quest_hook::libil2cpp::Void, 7usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    readWrite,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat_RenderTextureReadWrite_i32_4(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
        antiAliasing: i32,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    readWrite,
                    antiAliasing,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat_RenderTextureReadWrite_i32__cordl_bool3(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
        antiAliasing: i32,
        enableRandomWrite: bool,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                        i32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 9usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    readWrite,
                    antiAliasing,
                    enableRandomWrite,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat_RenderTextureReadWrite_i32__cordl_bool_RenderTextureMemoryless2(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                        i32,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                    ), quest_hook::libil2cpp::Void, 10usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    readWrite,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_i32_i32_i32_FilterMode_RenderTextureFormat_RenderTextureReadWrite_i32__cordl_bool_RenderTextureMemoryless__cordl_bool1(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
        useDynamicScale: bool,
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
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                        i32,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        bool,
                    ), quest_hook::libil2cpp::Void, 11usize>("GetTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTemporaryRT",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    readWrite,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                    useDynamicScale,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementUpdateCount(
        &mut self,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("IncrementUpdateCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementUpdateCount", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementUpdateCount_Injected(
        _unity_self: crate::System::IntPtr,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
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
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "IncrementUpdateCount_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IncrementUpdateCount_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitBuffer() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::System::IntPtr, 0usize>("InitBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitBuffer",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferCounterValue(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalSetComputeBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetComputeBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferCounterValue_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, u32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InternalSetComputeBufferCounterValue_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalSetComputeBufferCounterValue_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferData(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "InternalSetComputeBufferData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetComputeBufferData",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferData_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "InternalSetComputeBufferData_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetComputeBufferData_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferNativeData(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: crate::System::IntPtr,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        crate::System::IntPtr,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "InternalSetComputeBufferNativeData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetComputeBufferNativeData",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetComputeBufferNativeData_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        data: crate::System::IntPtr,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "InternalSetComputeBufferNativeData_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetComputeBufferNativeData_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferCounterValue(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "InternalSetGraphicsBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetGraphicsBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferCounterValue_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, u32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InternalSetGraphicsBufferCounterValue_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalSetGraphicsBufferCounterValue_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferData(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "InternalSetGraphicsBufferData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetGraphicsBufferData",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferData_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "InternalSetGraphicsBufferData_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetGraphicsBufferData_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferNativeData(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: crate::System::IntPtr,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::System::IntPtr,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "InternalSetGraphicsBufferNativeData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetGraphicsBufferNativeData",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetGraphicsBufferNativeData_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        data: crate::System::IntPtr,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
        elemSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "InternalSetGraphicsBufferNativeData_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSetGraphicsBufferNativeData_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                    elemSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BuildRayTracingAccelerationStructure(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
        buildSettings: crate::UnityEngine::Rendering::RayTracingAccelerationStructure_BuildSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure_BuildSettings,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Internal_BuildRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BuildRayTracingAccelerationStructure", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (accelerationStructure, buildSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BuildRayTracingAccelerationStructure_Injected(
        _unity_self: crate::System::IntPtr,
        accelerationStructure: crate::System::IntPtr,
        buildSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure_BuildSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure_BuildSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Internal_BuildRayTracingAccelerationStructure_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_BuildRayTracingAccelerationStructure_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, accelerationStructure, buildSettings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchCompute(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        threadGroupsX: i32,
        threadGroupsY: i32,
        threadGroupsZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_DispatchCompute"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchCompute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    threadGroupsX,
                    threadGroupsY,
                    threadGroupsZ,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchComputeIndirect(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_DispatchComputeIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchComputeIndirect",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchComputeIndirectGraphicsBuffer(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_DispatchComputeIndirectGraphicsBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchComputeIndirectGraphicsBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchComputeIndirectGraphicsBuffer_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        indirectBuffer: crate::System::IntPtr,
        argsOffset: u32,
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
                        u32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_DispatchComputeIndirectGraphicsBuffer_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchComputeIndirectGraphicsBuffer_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    indirectBuffer,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchComputeIndirect_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        indirectBuffer: crate::System::IntPtr,
        argsOffset: u32,
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
                        u32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_DispatchComputeIndirect_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchComputeIndirect_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    indirectBuffer,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchCompute_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        threadGroupsX: i32,
        threadGroupsY: i32,
        threadGroupsZ: i32,
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
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_DispatchCompute_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchCompute_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    threadGroupsX,
                    threadGroupsY,
                    threadGroupsZ,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchRays(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        rayGenShaderName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: u32,
        height: u32,
        depth: u32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                        u32,
                        u32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_DispatchRays"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchRays",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    rayTracingShader,
                    rayGenShaderName,
                    width,
                    height,
                    depth,
                    camera,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DispatchRays_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        rayGenShaderName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        width: u32,
        height: u32,
        depth: u32,
        camera: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        u32,
                        u32,
                        u32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DispatchRays_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DispatchRays_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    rayTracingShader,
                    rayGenShaderName,
                    width,
                    height,
                    depth,
                    camera,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>("Internal_DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMesh",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMeshInstanced"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstanced",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    matrices,
                    count,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedIndirect(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedIndirectGraphicsBuffer(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMeshInstancedIndirectGraphicsBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedIndirectGraphicsBuffer",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedIndirectGraphicsBuffer_Injected(
        _unity_self: crate::System::IntPtr,
        mesh: crate::System::IntPtr,
        submeshIndex: i32,
        material: crate::System::IntPtr,
        shaderPass: i32,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
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
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawMeshInstancedIndirectGraphicsBuffer_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedIndirectGraphicsBuffer_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedIndirect_Injected(
        _unity_self: crate::System::IntPtr,
        mesh: crate::System::IntPtr,
        submeshIndex: i32,
        material: crate::System::IntPtr,
        shaderPass: i32,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
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
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawMeshInstancedIndirect_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedIndirect_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedProcedural(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_DrawMeshInstancedProcedural"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, count, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstancedProcedural_Injected(
        _unity_self: crate::System::IntPtr,
        mesh: crate::System::IntPtr,
        submeshIndex: i32,
        material: crate::System::IntPtr,
        shaderPass: i32,
        count: i32,
        properties: crate::System::IntPtr,
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
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMeshInstancedProcedural_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstancedProcedural_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    count,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced_Injected(
        _unity_self: crate::System::IntPtr,
        mesh: crate::System::IntPtr,
        submeshIndex: i32,
        material: crate::System::IntPtr,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        count: i32,
        properties: crate::System::IntPtr,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawMeshInstanced_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMeshInstanced_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    matrices,
                    count,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh_Injected(
        _unity_self: crate::System::IntPtr,
        mesh: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        submeshIndex: i32,
        shaderPass: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMesh_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMesh_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    mesh,
                    matrix,
                    material,
                    submeshIndex,
                    shaderPass,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMultipleMeshes(
        &mut self,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>>,
        >,
        subsetIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        count: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawMultipleMeshes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMultipleMeshes",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrices,
                    meshes,
                    subsetIndices,
                    count,
                    material,
                    shaderPass,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMultipleMeshes_Injected(
        _unity_self: crate::System::IntPtr,
        matrices: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>>,
        >,
        subsetIndices: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        count: i32,
        material: crate::System::IntPtr,
        shaderPass: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawMultipleMeshes_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawMultipleMeshes_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    matrices,
                    meshes,
                    subsetIndices,
                    count,
                    material,
                    shaderPass,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawOcclusionMesh(
        &mut self,
        normalizedCamViewport: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::RectInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Internal_DrawOcclusionMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawOcclusionMesh", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (normalizedCamViewport))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawOcclusionMesh_Injected(
        _unity_self: crate::System::IntPtr,
        normalizedCamViewport: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_DrawOcclusionMesh_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawOcclusionMesh_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, normalizedCamViewport))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProcedural(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawProcedural"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProcedural",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexed(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProceduralIndexed"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexed",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexedIndirect(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProceduralIndexedIndirect",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexedIndirect",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexedIndirectGraphicsBuffer(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProceduralIndexedIndirectGraphicsBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexedIndirectGraphicsBuffer",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexedIndirectGraphicsBuffer_Injected(
        _unity_self: crate::System::IntPtr,
        indexBuffer: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Internal_DrawProceduralIndexedIndirectGraphicsBuffer_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexedIndirectGraphicsBuffer_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexedIndirect_Injected(
        _unity_self: crate::System::IntPtr,
        indexBuffer: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Internal_DrawProceduralIndexedIndirect_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexedIndirect_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndexed_Injected(
        _unity_self: crate::System::IntPtr,
        indexBuffer: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "Internal_DrawProceduralIndexed_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndexed_Injected",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndirect(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndirectGraphicsBuffer(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_DrawProceduralIndirectGraphicsBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndirectGraphicsBuffer",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndirectGraphicsBuffer_Injected(
        _unity_self: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProceduralIndirectGraphicsBuffer_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndirectGraphicsBuffer_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProceduralIndirect_Injected(
        _unity_self: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: crate::System::IntPtr,
        argsOffset: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        crate::System::IntPtr,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProceduralIndirect_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProceduralIndirect_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawProcedural_Injected(
        _unity_self: crate::System::IntPtr,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: crate::System::IntPtr,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
        properties: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        crate::System::IntPtr,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "Internal_DrawProcedural_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawProcedural_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRenderer(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_DrawRenderer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawRenderer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRendererList(
        &mut self,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RendererList),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Internal_DrawRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_DrawRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRendererList_Injected(
        _unity_self: crate::System::IntPtr,
        rendererList: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_DrawRendererList_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawRendererList_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRenderer_Injected(
        _unity_self: crate::System::IntPtr,
        renderer: crate::System::IntPtr,
        material: crate::System::IntPtr,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_DrawRenderer_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_DrawRenderer_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_unity_self, renderer, material, submeshIndex, shaderPass),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_RequestAsyncReadback_1"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_1",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, callback, nativeArrayData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_1_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_RequestAsyncReadback_1_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_1_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_2(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_RequestAsyncReadback_2"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_2",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (src, _cordl_size, offset, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_2_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_RequestAsyncReadback_2_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_2_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    _cordl_size,
                    offset,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_3(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_RequestAsyncReadback_3"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_3",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, callback, nativeArrayData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_3_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_RequestAsyncReadback_3_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_3_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_4(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_RequestAsyncReadback_4"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_4",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, mipIndex, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_4_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        mipIndex: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_RequestAsyncReadback_4_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_4_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, src, mipIndex, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_5(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_RequestAsyncReadback_5"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_5",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (src, mipIndex, dstFormat, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_5_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_RequestAsyncReadback_5_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_5_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    mipIndex,
                    dstFormat,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_6(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "Internal_RequestAsyncReadback_6"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_6",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    src,
                    mipIndex,
                    x,
                    width,
                    y,
                    height,
                    z,
                    depth,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_6_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "Internal_RequestAsyncReadback_6_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_6_Injected",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    mipIndex,
                    x,
                    width,
                    y,
                    height,
                    z,
                    depth,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_7(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "Internal_RequestAsyncReadback_7"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_7",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    src,
                    mipIndex,
                    x,
                    width,
                    y,
                    height,
                    z,
                    depth,
                    dstFormat,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_7_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 12usize>(
                        "Internal_RequestAsyncReadback_7_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_7_Injected",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    mipIndex,
                    x,
                    width,
                    y,
                    height,
                    z,
                    depth,
                    dstFormat,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_8(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_RequestAsyncReadback_8"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_8",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, callback, nativeArrayData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_8_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_RequestAsyncReadback_8_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_8_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, src, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_9(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_RequestAsyncReadback_9"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_9",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (src, _cordl_size, offset, callback, nativeArrayData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RequestAsyncReadback_9_Injected(
        _unity_self: crate::System::IntPtr,
        src: crate::System::IntPtr,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
        nativeArrayData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_RequestAsyncReadback_9_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RequestAsyncReadback_9_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    src,
                    _cordl_size,
                    offset,
                    callback,
                    nativeArrayData,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeBufferParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        nameID: i32,
        buffer: crate::System::IntPtr,
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
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeBufferParam_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_unity_self, computeShader, kernelIndex, nameID, buffer),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeConstantComputeBufferParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeConstantComputeBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeConstantComputeBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeConstantComputeBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
        offset: i32,
        _cordl_size: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_SetComputeConstantComputeBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeConstantComputeBufferParam_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    nameID,
                    buffer,
                    offset,
                    _cordl_size,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeConstantGraphicsBufferParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeConstantGraphicsBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeConstantGraphicsBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeConstantGraphicsBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
        offset: i32,
        _cordl_size: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_SetComputeConstantGraphicsBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeConstantGraphicsBufferParam_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    nameID,
                    buffer,
                    offset,
                    _cordl_size,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeFloats(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetComputeFloats"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeFloats",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeFloats_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeFloats_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeFloats_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeGraphicsBufferHandleParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeGraphicsBufferHandleParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeGraphicsBufferHandleParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeGraphicsBufferHandleParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        nameID: i32,
        bufferHandle: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
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
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeGraphicsBufferHandleParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeGraphicsBufferHandleParam_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    nameID,
                    bufferHandle,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeGraphicsBufferParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeGraphicsBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeGraphicsBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeGraphicsBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        nameID: i32,
        buffer: crate::System::IntPtr,
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
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeGraphicsBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeGraphicsBufferParam_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_unity_self, computeShader, kernelIndex, nameID, buffer),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeInts(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetComputeInts"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeInts",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeInts_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeInts_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeInts_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeRayTracingAccelerationStructure(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetComputeRayTracingAccelerationStructure",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeRayTracingAccelerationStructure",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, nameID, accelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeRayTracingAccelerationStructure_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        nameID: i32,
        accelerationStructure: crate::System::IntPtr,
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
                        i32,
                        crate::System::IntPtr,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetComputeRayTracingAccelerationStructure_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeRayTracingAccelerationStructure_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    nameID,
                    accelerationStructure,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeTextureParam(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeTextureParam",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, nameID, rt, mipLevel, element),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetComputeTextureParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        kernelIndex: i32,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
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
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "Internal_SetComputeTextureParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetComputeTextureParam_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    computeShader,
                    kernelIndex,
                    nameID,
                    rt,
                    mipLevel,
                    element,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingAccelerationStructure(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingAccelerationStructure",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingAccelerationStructure",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, nameID, accelerationStructure))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingAccelerationStructure_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        accelerationStructure: crate::System::IntPtr,
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
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingAccelerationStructure_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingAccelerationStructure_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (_unity_self, rayTracingShader, nameID, accelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingComputeBufferParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingComputeBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingComputeBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingComputeBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
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
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingComputeBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingComputeBufferParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingConstantComputeBufferParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetRayTracingConstantComputeBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingConstantComputeBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingConstantComputeBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
        offset: i32,
        _cordl_size: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_SetRayTracingConstantComputeBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingConstantComputeBufferParam_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    rayTracingShader,
                    nameID,
                    buffer,
                    offset,
                    _cordl_size,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingConstantGraphicsBufferParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SetRayTracingConstantGraphicsBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingConstantGraphicsBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingConstantGraphicsBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
        offset: i32,
        _cordl_size: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "Internal_SetRayTracingConstantGraphicsBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingConstantGraphicsBufferParam_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    rayTracingShader,
                    nameID,
                    buffer,
                    offset,
                    _cordl_size,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingFloatParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingFloatParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Internal_SetRayTracingFloatParam_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetRayTracingFloatParam_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, rayTracingShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingFloats(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingFloats"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingFloats",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingFloats_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingFloats_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingFloats_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingGraphicsBufferHandleParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingGraphicsBufferHandleParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingGraphicsBufferHandleParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingGraphicsBufferHandleParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        bufferHandle: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
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
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::GraphicsBufferHandle>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingGraphicsBufferHandleParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingGraphicsBufferHandleParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingGraphicsBufferParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingGraphicsBufferParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingGraphicsBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingGraphicsBufferParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        buffer: crate::System::IntPtr,
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
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingGraphicsBufferParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingGraphicsBufferParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingIntParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingIntParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingIntParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, i32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Internal_SetRayTracingIntParam_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetRayTracingIntParam_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, rayTracingShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingInts(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingInts"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingInts",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingInts_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingInts_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingInts_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingMatrixArrayParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingMatrixArrayParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingMatrixArrayParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingMatrixArrayParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingMatrixArrayParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingMatrixParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingMatrixParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        val: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
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
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingMatrixParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingMatrixParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, rayTracingShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingTextureParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingTextureParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingTextureParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingTextureParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingTextureParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, rayTracingShader, nameID, rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingVectorArrayParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingVectorArrayParam",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingVectorArrayParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingVectorArrayParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingVectorArrayParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingVectorParam(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SetRayTracingVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRayTracingVectorParam_Injected(
        _unity_self: crate::System::IntPtr,
        rayTracingShader: crate::System::IntPtr,
        nameID: i32,
        val: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
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
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "Internal_SetRayTracingVectorParam_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetRayTracingVectorParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, rayTracingShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetSinglePassStereo(
        &mut self,
        mode: crate::UnityEngine::Rendering::SinglePassStereoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SinglePassStereoMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Internal_SetSinglePassStereo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetSinglePassStereo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetSinglePassStereo_Injected(
        _unity_self: crate::System::IntPtr,
        mode: crate::UnityEngine::Rendering::SinglePassStereoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::SinglePassStereoMode,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_SetSinglePassStereo_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SetSinglePassStereo_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnRenderObjectCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "InvokeOnRenderObjectCallbacks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeOnRenderObjectCallbacks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnRenderObjectCallbacks_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "InvokeOnRenderObjectCallbacks_Internal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeOnRenderObjectCallbacks_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnRenderObjectCallbacks_Internal_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("InvokeOnRenderObjectCallbacks_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeOnRenderObjectCallbacks_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomBlit(
        &mut self,
        callback: crate::System::IntPtr,
        command: u32,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        commandParam: u32,
        commandFlags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        u32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        u32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "IssuePluginCustomBlit"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomBlit",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (callback, command, source, dest, commandParam, commandFlags),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomBlitInternal(
        &mut self,
        callback: crate::System::IntPtr,
        command: u32,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        commandParam: u32,
        commandFlags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        u32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "IssuePluginCustomBlitInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomBlitInternal",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (callback, command, source, dest, commandParam, commandFlags),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomBlitInternal_Injected(
        _unity_self: crate::System::IntPtr,
        callback: crate::System::IntPtr,
        command: u32,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        commandParam: u32,
        commandFlags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        u32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "IssuePluginCustomBlitInternal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomBlitInternal_Injected",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    callback,
                    command,
                    source,
                    dest,
                    commandParam,
                    commandFlags,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomTextureUpdateInternal(
        &mut self,
        callback: crate::System::IntPtr,
        targetTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        userData: u32,
        useNewUnityRenderingExtTextureUpdateParamsV2: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        u32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "IssuePluginCustomTextureUpdateInternal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomTextureUpdateInternal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    callback,
                    targetTexture,
                    userData,
                    useNewUnityRenderingExtTextureUpdateParamsV2,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomTextureUpdateInternal_Injected(
        _unity_self: crate::System::IntPtr,
        callback: crate::System::IntPtr,
        targetTexture: crate::System::IntPtr,
        userData: u32,
        useNewUnityRenderingExtTextureUpdateParamsV2: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        u32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "IssuePluginCustomTextureUpdateInternal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomTextureUpdateInternal_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    callback,
                    targetTexture,
                    userData,
                    useNewUnityRenderingExtTextureUpdateParamsV2,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomTextureUpdateV2(
        &mut self,
        callback: crate::System::IntPtr,
        targetTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        userData: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "IssuePluginCustomTextureUpdateV2"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomTextureUpdateV2",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (callback, targetTexture, userData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEvent(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IssuePluginEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventAndData(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("IssuePluginEventAndData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEventAndData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventAndDataInternal(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("IssuePluginEventAndDataInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEventAndDataInternal", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventAndDataInternal_Injected(
        _unity_self: crate::System::IntPtr,
        callback: crate::System::IntPtr,
        eventID: i32,
        data: crate::System::IntPtr,
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
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "IssuePluginEventAndDataInternal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginEventAndDataInternal_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, callback, eventID, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventInternal(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IssuePluginEventInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEventInternal", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventInternal_Injected(
        _unity_self: crate::System::IntPtr,
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("IssuePluginEventInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEventInternal_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, callback, eventID))? };
        Ok(__cordl_ret.into())
    }
    pub fn MarkLateLatchMatrixShaderPropertyID(
        &mut self,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
        shaderPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "MarkLateLatchMatrixShaderPropertyID"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MarkLateLatchMatrixShaderPropertyID",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (matrixPropertyType, shaderPropertyID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkLateLatchMatrixShaderPropertyID_Injected(
        _unity_self: crate::System::IntPtr,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
        shaderPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "MarkLateLatchMatrixShaderPropertyID_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MarkLateLatchMatrixShaderPropertyID_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, matrixPropertyType, shaderPropertyID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NextSubPass(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("NextSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NextSubPass",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextSubPass_Internal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("NextSubPass_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NextSubPass_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn NextSubPass_Internal_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("NextSubPass_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NextSubPass_Internal_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn Release(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Release",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseBuffer(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ReleaseBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseBuffer",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseBuffer_Injected(
        _unity_self: crate::System::IntPtr,
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
                    >("ReleaseBuffer_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseBuffer_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseTemporaryRT(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("ReleaseTemporaryRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseTemporaryRT",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseTemporaryRT_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ReleaseTemporaryRT_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseTemporaryRT_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_ComputeBuffer_Action_1_0<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_ComputeBuffer_i32_i32_Action_1_1<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, _cordl_size, offset, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_GraphicsBuffer_Action_1_2<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_GraphicsBuffer_i32_i32_Action_1_3<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, _cordl_size, offset, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_Action_1_4<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_Action_1_5<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, mipIndex, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_GraphicsFormat_Action_1_7<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, mipIndex, dstFormat, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_TextureFormat_Action_1_6<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::TextureFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        crate::UnityEngine::TextureFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, mipIndex, dstFormat, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_Action_1_8<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_GraphicsFormat_Action_1_10<
        T,
    >(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, dstFormat, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_TextureFormat_Action_1_9<
        T,
    >(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::TextureFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::TextureFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, dstFormat, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_ComputeBuffer0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_GraphicsBuffer1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_Array0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_Array_i32_i32_i32_3(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_List_1_1<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_List_1_i32_i32_i32_4<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_NativeArray_1_2<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_NativeArray_1_i32_i32_i32_5<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_Array6(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_Array_i32_i32_i32_9(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_List_1_7<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_List_1_i32_i32_i32_10<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_NativeArray_1_8<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_NativeArray_1_i32_i32_i32_11<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_GraphicsBuffer5(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_GraphicsBufferHandle3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_ComputeBuffer0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_GraphicsBuffer4(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_GraphicsBufferHandle2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_Il2CppString_GraphicsBuffer3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_i32_ComputeBuffer0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_i32_GraphicsBuffer2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeFloatParam_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeFloatParam_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, i32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeIntParam_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeIntParam_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeKeyword(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetComputeKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeKeyword_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixArrayParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixArrayParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeMatrixArrayParam_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixArrayParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixArrayParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        val: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
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
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeMatrixParam_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString_i32_2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, rt, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString_i32_RenderTextureSubElement4(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, name, rt, mipLevel, element),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, nameID, rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_i32_3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, rt, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_i32_RenderTextureSubElement5(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, nameID, rt, mipLevel, element),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorArrayParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorArrayParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeVectorArrayParam_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorArrayParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorArrayParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_Injected(
        _unity_self: crate::System::IntPtr,
        computeShader: crate::System::IntPtr,
        nameID: i32,
        val: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
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
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeVectorParam_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorParam_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetExecutionFlags(
        &mut self,
        flags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::CommandBufferExecutionFlags),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetExecutionFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetExecutionFlags", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetExecutionFlags_Injected(
        _unity_self: crate::System::IntPtr,
        flags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetExecutionFlags_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetExecutionFlags_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, flags))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetFoveatedRenderingMode(
        &mut self,
        foveatedRenderingMode: crate::UnityEngine::Rendering::FoveatedRenderingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::FoveatedRenderingMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetFoveatedRenderingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetFoveatedRenderingMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (foveatedRenderingMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetFoveatedRenderingMode_Injected(
        _unity_self: crate::System::IntPtr,
        foveatedRenderingMode: crate::UnityEngine::Rendering::FoveatedRenderingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::FoveatedRenderingMode,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetFoveatedRenderingMode_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetFoveatedRenderingMode_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, foveatedRenderingMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBufferInternal(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalBufferInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBufferInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBufferInternal_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalBufferInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalBufferInternal_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_ComputeBuffer0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_GraphicsBuffer2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_ComputeBuffer1(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_GraphicsBuffer3(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalColor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalColor_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalColor_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalColor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBufferInternal(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBufferInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBufferInternal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBufferInternal_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetGlobalConstantBufferInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalConstantBufferInternal_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_ComputeBuffer_Il2CppString1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, name, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_ComputeBuffer_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_GraphicsBuffer_Il2CppString3(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, name, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_GraphicsBuffer_i32_2(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantGraphicsBufferInternal(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantGraphicsBufferInternal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantGraphicsBufferInternal",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantGraphicsBufferInternal_Injected(
        _unity_self: crate::System::IntPtr,
        buffer: crate::System::IntPtr,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetGlobalConstantGraphicsBufferInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalConstantGraphicsBufferInternal_Injected", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalDepthBias(
        &mut self,
        bias: f32,
        slopeBias: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalDepthBias",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalDepthBias",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (bias, slopeBias))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalDepthBias_Injected(
        _unity_self: crate::System::IntPtr,
        bias: f32,
        slopeBias: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, f32, f32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalDepthBias_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalDepthBias_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, bias, slopeBias))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArrayListImpl(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArrayListImpl"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArrayListImpl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArrayListImpl_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalFloatArrayListImpl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArrayListImpl_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalFloatArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, f32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalFloat_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalFloat_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_i32_0(
        &mut self,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalGraphicsBufferInternal(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalGraphicsBufferInternal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalGraphicsBufferInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalGraphicsBufferInternal_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalGraphicsBufferInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalGraphicsBufferInternal_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalInt_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalInt_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_i32_0(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInteger",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, i32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetGlobalInteger_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalInteger_Injected", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_i32_0(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalInteger",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInteger",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalKeyword(
        &mut self,
        keyword: crate::UnityEngine::Rendering::GlobalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::GlobalKeyword, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalKeyword", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalKeyword_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArrayListImpl(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArrayListImpl"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArrayListImpl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArrayListImpl_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalMatrixArrayListImpl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArrayListImpl_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalMatrixArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrix",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalMatrix_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrix_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString_RenderTextureSubElement2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Impl(
        &mut self,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalTexture_Impl"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture_Impl",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, rt, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Impl_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalTexture_Impl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture_Impl_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, rt, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalTexture", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTextureSubElement3(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArrayListImpl(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArrayListImpl"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArrayListImpl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArrayListImpl_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalVectorArrayListImpl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArrayListImpl_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        values: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalVectorArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalVector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVector",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Injected(
        _unity_self: crate::System::IntPtr,
        nameID: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetGlobalVector_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVector_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Vector4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalVector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalVector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInstanceMultiplier(
        &mut self,
        multiplier: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(
                        "SetInstanceMultiplier",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInstanceMultiplier",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (multiplier))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInstanceMultiplier_Injected(
        _unity_self: crate::System::IntPtr,
        multiplier: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, u32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetInstanceMultiplier_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetInstanceMultiplier_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, multiplier))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInvertCulling(
        &mut self,
        invertCulling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetInvertCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInvertCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (invertCulling))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInvertCulling_Injected(
        _unity_self: crate::System::IntPtr,
        invertCulling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetInvertCulling_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetInvertCulling_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, invertCulling))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_ByRefMut__cordl_bool0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_ComputeShader_ByRefMut__cordl_bool2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_Material_ByRefMut__cordl_bool1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetLateLatchProjectionMatrices(
        &mut self,
        projectionMat: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "SetLateLatchProjectionMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetLateLatchProjectionMatrices",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (projectionMat))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetLateLatchProjectionMatrices_Injected(
        _unity_self: crate::System::IntPtr,
        projectionMat: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
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
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetLateLatchProjectionMatrices_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetLateLatchProjectionMatrices_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, projectionMat))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMaterialKeyword(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: crate::UnityEngine::Rendering::LocalKeyword,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        crate::UnityEngine::Rendering::LocalKeyword,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetMaterialKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMaterialKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMaterialKeyword_Injected(
        _unity_self: crate::System::IntPtr,
        material: crate::System::IntPtr,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetMaterialKeyword_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMaterialKeyword_Injected",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, material, keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetProjectionMatrix(
        &mut self,
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetProjectionMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetProjectionMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetProjectionMatrix_Injected(
        _unity_self: crate::System::IntPtr,
        proj: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetProjectionMatrix_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetProjectionMatrix_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_GraphicsBuffer(
        &mut self,
        index: i32,
        uav: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        preserveCounterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRandomWriteTarget_GraphicsBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomWriteTarget_GraphicsBuffer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index, uav, preserveCounterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_GraphicsBuffer2(
        &mut self,
        index: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetRandomWriteTarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomWriteTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_GraphicsBuffer_Injected(
        _unity_self: crate::System::IntPtr,
        index: i32,
        uav: crate::System::IntPtr,
        preserveCounterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr, bool),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRandomWriteTarget_GraphicsBuffer_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRandomWriteTarget_GraphicsBuffer_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (_unity_self, index, uav, preserveCounterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_GraphicsBuffer__cordl_bool1(
        &mut self,
        index: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        preserveCounterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRandomWriteTarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomWriteTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index, buffer, preserveCounterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_RenderTargetIdentifier0(
        &mut self,
        index: i32,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetRandomWriteTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRandomWriteTarget", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_Texture(
        &mut self,
        index: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetRandomWriteTarget_Texture"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomWriteTarget_Texture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget_Texture_Injected(
        _unity_self: crate::System::IntPtr,
        index: i32,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRandomWriteTarget_Texture_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRandomWriteTarget_Texture_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, index, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_ComputeShader_i32_Il2CppString_RayTracingAccelerationStructure2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    name,
                    rayTracingAccelerationStructure,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_ComputeShader_i32_i32_RayTracingAccelerationStructure3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    nameID,
                    rayTracingAccelerationStructure,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_RayTracingShader_Il2CppString_RayTracingAccelerationStructure0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, name, rayTracingAccelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_RayTracingShader_i32_RayTracingAccelerationStructure1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, rayTracingAccelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_ComputeBuffer0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_GraphicsBuffer2(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_GraphicsBufferHandle4(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_ComputeBuffer1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_GraphicsBuffer3(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_GraphicsBufferHandle5(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_Il2CppString_GraphicsBuffer3(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_i32_ComputeBuffer0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_i32_GraphicsBuffer2(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParams_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParams_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParams_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParams_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixArrayParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixArrayParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingTextureParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingTextureParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingTextureParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingTextureParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorArrayParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorArrayParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetColorDepthSubtarget(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "SetRenderTargetColorDepthSubtarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetColorDepthSubtarget",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    color,
                    depth,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                    mipLevel,
                    cubemapFace,
                    depthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetColorDepthSubtarget_Injected(
        _unity_self: crate::System::IntPtr,
        color: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        depth: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "SetRenderTargetColorDepthSubtarget_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetColorDepthSubtarget_Injected",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    color,
                    depth,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                    mipLevel,
                    cubemapFace,
                    depthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetColorDepth_Internal(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        flags: crate::UnityEngine::Rendering::RenderTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetFlags,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "SetRenderTargetColorDepth_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetColorDepth_Internal",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    color,
                    depth,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                    flags,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetColorDepth_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        color: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        depth: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        flags: crate::UnityEngine::Rendering::RenderTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetFlags,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "SetRenderTargetColorDepth_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetColorDepth_Internal_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    color,
                    depth,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                    flags,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetMultiSubtarget(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadActions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderBufferLoadAction,
            >,
        >,
        colorStoreActions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderBufferStoreAction,
            >,
        >,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "SetRenderTargetMultiSubtarget"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetMultiSubtarget",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    colors,
                    depth,
                    colorLoadActions,
                    colorStoreActions,
                    depthLoadAction,
                    depthStoreAction,
                    mipLevel,
                    cubemapFace,
                    depthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetMultiSubtarget_Injected(
        _unity_self: crate::System::IntPtr,
        colors: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        depth: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadActions: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        colorStoreActions: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "SetRenderTargetMultiSubtarget_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetMultiSubtarget_Injected",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    colors,
                    depth,
                    colorLoadActions,
                    colorStoreActions,
                    depthLoadAction,
                    depthStoreAction,
                    mipLevel,
                    cubemapFace,
                    depthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetMulti_Internal(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadActions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderBufferLoadAction,
            >,
        >,
        colorStoreActions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderBufferStoreAction,
            >,
        >,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        flags: crate::UnityEngine::Rendering::RenderTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderBufferLoadAction,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderBufferStoreAction,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetFlags,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "SetRenderTargetMulti_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetMulti_Internal",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    colors,
                    depth,
                    colorLoadActions,
                    colorStoreActions,
                    depthLoadAction,
                    depthStoreAction,
                    flags,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetMulti_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        colors: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        depth: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadActions: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        colorStoreActions: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        flags: crate::UnityEngine::Rendering::RenderTargetFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetFlags,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "SetRenderTargetMulti_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetMulti_Internal_Injected",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    colors,
                    depth,
                    colorLoadActions,
                    colorStoreActions,
                    depthLoadAction,
                    depthStoreAction,
                    flags,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRenderTargetSingle_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetSingle_Internal",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetRenderTargetSingle_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTargetSingle_Internal_Injected",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    _unity_self,
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier11(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colors, depth))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier_i32_CubemapFace_i32_12(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (colors, depth, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetBinding14(
        &mut self,
        binding: crate::UnityEngine::Rendering::RenderTargetBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetBinding),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (binding))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetBinding_i32_CubemapFace_i32_13(
        &mut self,
        binding: crate::UnityEngine::Rendering::RenderTargetBinding,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetBinding,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (binding, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier0(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction1(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, loadAction, storeAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderBufferLoadAction_RenderBufferStoreAction2(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction10(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 6usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    color,
                    colorLoadAction,
                    colorStoreAction,
                    depth,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier6(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (color, depth))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_7(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (color, depth, mipLevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_CubemapFace8(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (color, depth, mipLevel, cubemapFace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_CubemapFace_i32_9(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (color, depth, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_3(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, mipLevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_CubemapFace4(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, mipLevel, cubemapFace))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_CubemapFace_i32_5(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rt, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowSamplingMode(
        &mut self,
        shadowmap: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mode: crate::UnityEngine::Rendering::ShadowSamplingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetShadowSamplingMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetShadowSamplingMode",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (shadowmap, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowSamplingMode_Impl(
        &mut self,
        shadowmap: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mode: crate::UnityEngine::Rendering::ShadowSamplingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetShadowSamplingMode_Impl"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetShadowSamplingMode_Impl",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (shadowmap, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowSamplingMode_Impl_Injected(
        _unity_self: crate::System::IntPtr,
        shadowmap: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mode: crate::UnityEngine::Rendering::ShadowSamplingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetShadowSamplingMode_Impl_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetShadowSamplingMode_Impl_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, shadowmap, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetSinglePassStereo(
        &mut self,
        mode: crate::UnityEngine::Rendering::SinglePassStereoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SinglePassStereoMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetSinglePassStereo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSinglePassStereo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewMatrix(
        &mut self,
        view: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetViewMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetViewMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (view))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewMatrix_Injected(
        _unity_self: crate::System::IntPtr,
        view: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetViewMatrix_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewMatrix_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, view))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices(
        &mut self,
        view: crate::UnityEngine::Matrix4x4,
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetViewProjectionMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetViewProjectionMatrices", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (view, proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices_Injected(
        _unity_self: crate::System::IntPtr,
        view: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        proj: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetViewProjectionMatrices_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewProjectionMatrices_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, view, proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewport(
        &mut self,
        pixelRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rect), quest_hook::libil2cpp::Void, 1usize>(
                        "SetViewport",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewport",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pixelRect))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewport_Injected(
        _unity_self: crate::System::IntPtr,
        pixelRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetViewport_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewport_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, pixelRect))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetWireframe(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetWireframe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetWireframe",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (enable))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetWireframe_Injected(
        _unity_self: crate::System::IntPtr,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetWireframe_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetWireframe_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, enable))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties(
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
                    >("SetupCameraProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupCameraProperties", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_Internal(
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
                    >("SetupCameraProperties_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupCameraProperties_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        camera: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetupCameraProperties_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupCameraProperties_Internal_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnmarkLateLatchMatrix(
        &mut self,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::CameraLateLatchMatrixType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnmarkLateLatchMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnmarkLateLatchMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (matrixPropertyType))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnmarkLateLatchMatrix_Injected(
        _unity_self: crate::System::IntPtr,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnmarkLateLatchMatrix_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnmarkLateLatchMatrix_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, matrixPropertyType))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAgainstExecutionFlags(
        &mut self,
        requiredFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
        invalidFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                        crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                    ), bool, 2usize>("ValidateAgainstExecutionFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateAgainstExecutionFlags",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (requiredFlags, invalidFlags))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAgainstExecutionFlags_Injected(
        _unity_self: crate::System::IntPtr,
        requiredFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
        invalidFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                        crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                    ), bool, 3usize>("ValidateAgainstExecutionFlags_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateAgainstExecutionFlags_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (_unity_self, requiredFlags, invalidFlags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnAsyncGraphicsFence_GraphicsFence0(
        &mut self,
        fence: crate::UnityEngine::Rendering::GraphicsFence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::GraphicsFence),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WaitOnAsyncGraphicsFence")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WaitOnAsyncGraphicsFence", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fence))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnAsyncGraphicsFence_SynchronisationStage1(
        &mut self,
        fence: crate::UnityEngine::Rendering::GraphicsFence,
        stage: crate::UnityEngine::Rendering::SynchronisationStage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::GraphicsFence,
                        crate::UnityEngine::Rendering::SynchronisationStage,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "WaitOnAsyncGraphicsFence"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitOnAsyncGraphicsFence",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fence, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnAsyncGraphicsFence_SynchronisationStageFlags2(
        &mut self,
        fence: crate::UnityEngine::Rendering::GraphicsFence,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::GraphicsFence,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "WaitOnAsyncGraphicsFence"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitOnAsyncGraphicsFence",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fence, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnGPUFence_Internal(
        &mut self,
        fencePtr: crate::System::IntPtr,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "WaitOnGPUFence_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitOnGPUFence_Internal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fencePtr, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnGPUFence_Internal_Injected(
        _unity_self: crate::System::IntPtr,
        fencePtr: crate::System::IntPtr,
        stage: crate::UnityEngine::Rendering::SynchronisationStageFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        crate::System::IntPtr,
                        crate::UnityEngine::Rendering::SynchronisationStageFlags,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "WaitOnGPUFence_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitOnGPUFence_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, fencePtr, stage))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_name", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_name_Injected(
        _unity_self: crate::System::IntPtr,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("get_name_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_name_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_sizeInBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sizeInBytes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sizeInBytes_Injected(
        _unity_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::IntPtr), i32, 1usize>(
                        "get_sizeInBytes_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_sizeInBytes_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (_unity_self))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_name", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_name_Injected(
        _unity_self: crate::System::IntPtr,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("set_name_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_name_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_unity_self, value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBuffer_BindingsMarshaller {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CommandBuffer/BindingsMarshaller";
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
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
impl crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller {
    pub fn ConvertToNative(
        commandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        crate::System::IntPtr,
                        1usize,
                    >("ConvertToNative")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertToNative", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr =
            unsafe { cordl_method_info.invoke_unchecked((), (commandBuffer))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBuffer+BindingsMarshaller")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::CommandBuffer_BindingsMarshaller
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

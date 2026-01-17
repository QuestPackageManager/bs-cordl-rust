#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBufferExtensions")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct CommandBufferExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBufferExtensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::CommandBufferExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CommandBufferExtensions";
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
#[cfg(feature = "UnityEngine+Rendering+CommandBufferExtensions")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CommandBufferExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBufferExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CommandBufferExtensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBufferExtensions")]
impl crate::UnityEngine::Rendering::CommandBufferExtensions {
    pub fn Internal_SwitchIntoFastMemory(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        fastMemoryFlags: crate::UnityEngine::Rendering::FastMemoryFlags,
        residency: f32,
        copyContents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        crate::UnityEngine::Rendering::FastMemoryFlags,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SwitchIntoFastMemory"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SwitchIntoFastMemory",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, rt, fastMemoryFlags, residency, copyContents))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SwitchIntoFastMemory_Injected(
        cmd: crate::System::IntPtr,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        fastMemoryFlags: crate::UnityEngine::Rendering::FastMemoryFlags,
        residency: f32,
        copyContents: bool,
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
                        crate::UnityEngine::Rendering::FastMemoryFlags,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "Internal_SwitchIntoFastMemory_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SwitchIntoFastMemory_Injected",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, rt, fastMemoryFlags, residency, copyContents))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SwitchOutOfFastMemory(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        copyContents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SwitchOutOfFastMemory"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SwitchOutOfFastMemory",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, rt, copyContents))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SwitchOutOfFastMemory_Injected(
        cmd: crate::System::IntPtr,
        rt: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
        copyContents: bool,
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
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "Internal_SwitchOutOfFastMemory_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_SwitchOutOfFastMemory_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, rt, copyContents))? };
        Ok(__cordl_ret.into())
    }
    pub fn SwitchIntoFastMemory(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        rid: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        fastMemoryFlags: crate::UnityEngine::Rendering::FastMemoryFlags,
        residency: f32,
        copyContents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::FastMemoryFlags,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SwitchIntoFastMemory"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SwitchIntoFastMemory",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, rid, fastMemoryFlags, residency, copyContents))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SwitchOutOfFastMemory(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        rid: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        copyContents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SwitchOutOfFastMemory"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SwitchOutOfFastMemory",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, rid, copyContents))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CommandBufferExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CommandBufferExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

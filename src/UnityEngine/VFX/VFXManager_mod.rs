#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXManager")]
#[repr(C)]
#[derive(Debug)]
pub struct VFXManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::VFX::VFXManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.VFX";
    const CLASS_NAME: &'static str = "VFXManager";
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
#[cfg(feature = "UnityEngine+VFX+VFXManager")]
impl std::ops::Deref for crate::UnityEngine::VFX::VFXManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VFXManager")]
impl std::ops::DerefMut for crate::UnityEngine::VFX::VFXManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+VFX+VFXManager")]
impl crate::UnityEngine::VFX::VFXManager {
    pub fn Internal_ProcessCameraCommand(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        camXRSettings: crate::UnityEngine::VFX::VFXCameraXRSettings,
        cullResults: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::VFX::VFXCameraXRSettings,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Internal_ProcessCameraCommand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_ProcessCameraCommand", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, cmd, camXRSettings, cullResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ProcessCameraCommand_Injected(
        cam: crate::System::IntPtr,
        cmd: crate::System::IntPtr,
        camXRSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::VFX::VFXCameraXRSettings,
        >,
        cullResults: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::VFX::VFXCameraXRSettings,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Internal_ProcessCameraCommand_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_ProcessCameraCommand_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, cmd, camXRSettings, cullResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCameraBufferNeeded(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::VFX::VFXCameraBufferTypes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::VFX::VFXCameraBufferTypes,
                        1usize,
                    >("IsCameraBufferNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCameraBufferNeeded", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::VFX::VFXCameraBufferTypes = unsafe {
            cordl_method_info.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCameraBufferNeeded_Injected(
        cam: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::VFX::VFXCameraBufferTypes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        crate::UnityEngine::VFX::VFXCameraBufferTypes,
                        1usize,
                    >("IsCameraBufferNeeded_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCameraBufferNeeded_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::VFX::VFXCameraBufferTypes = unsafe {
            cordl_method_info.invoke_unchecked((), (cam))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCameraCommand(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        camXRSettings: crate::UnityEngine::VFX::VFXCameraXRSettings,
        results: crate::UnityEngine::Rendering::CullingResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::VFX::VFXCameraXRSettings,
                            crate::UnityEngine::Rendering::CullingResults,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ProcessCameraCommand")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessCameraCommand", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cam, cmd, camXRSettings, results))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraBuffer(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        _cordl_type: crate::UnityEngine::VFX::VFXCameraBufferTypes,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                            crate::UnityEngine::VFX::VFXCameraBufferTypes,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetCameraBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCameraBuffer", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, _cordl_type, buffer, x, y, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCameraBuffer_Injected(
        cam: crate::System::IntPtr,
        _cordl_type: crate::UnityEngine::VFX::VFXCameraBufferTypes,
        buffer: crate::System::IntPtr,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            crate::UnityEngine::VFX::VFXCameraBufferTypes,
                            crate::System::IntPtr,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("SetCameraBuffer_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCameraBuffer_Injected", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cam, _cordl_type, buffer, x, y, width, height))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+VFX+VFXManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::VFX::VFXManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

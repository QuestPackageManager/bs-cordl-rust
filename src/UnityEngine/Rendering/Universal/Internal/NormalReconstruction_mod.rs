#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
#[repr(C)]
#[derive(Debug)]
pub struct NormalReconstruction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Internal::NormalReconstruction
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "NormalReconstruction";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::Internal::NormalReconstruction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Internal::NormalReconstruction
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
impl crate::UnityEngine::Rendering::Universal::Internal::NormalReconstruction {
    pub fn SetupProperties_CommandBuffer_ByRefMut0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupProperties_CommandBuffer_UniversalCameraData2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupProperties_RasterCommandBuffer_ByRefMut1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupProperties_RasterCommandBuffer_ByRefMut3(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupProperties",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, cameraData))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+NormalReconstruction")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Internal::NormalReconstruction
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

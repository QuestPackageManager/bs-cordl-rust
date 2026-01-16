#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentDepthUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::EnvironmentDepth::EnvironmentDepthUtils
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.EnvironmentDepth";
    const CLASS_NAME: &'static str = "EnvironmentDepthUtils";
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
#[cfg(feature = "Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
impl std::ops::Deref for crate::Meta::XR::EnvironmentDepth::EnvironmentDepthUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
impl std::ops::DerefMut for crate::Meta::XR::EnvironmentDepth::EnvironmentDepthUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
impl crate::Meta::XR::EnvironmentDepth::EnvironmentDepthUtils {
    pub fn CalculateDepthCameraMatrices(
        frameDesc: crate::Meta::XR::EnvironmentDepth::DepthFrameDesc,
        projMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        viewMatrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Meta::XR::EnvironmentDepth::DepthFrameDesc,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "CalculateDepthCameraMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateDepthCameraMatrices",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (frameDesc, projMatrix, viewMatrix))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateReprojection(
        frameDesc: crate::Meta::XR::EnvironmentDepth::DepthFrameDesc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Meta::XR::EnvironmentDepth::DepthFrameDesc),
                        crate::UnityEngine::Matrix4x4,
                        1usize,
                    >("CalculateReprojection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateReprojection", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 =
            unsafe { cordl_method_info.invoke_unchecked((), (frameDesc))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeNdcToLinearDepthParameters(
        near: f32,
        far: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32), crate::UnityEngine::Vector4, 2usize>(
                        "ComputeNdcToLinearDepthParameters",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeNdcToLinearDepthParameters",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 =
            unsafe { cordl_method_info.invoke_unchecked((), (near, far))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+EnvironmentDepth+EnvironmentDepthUtils")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::EnvironmentDepth::EnvironmentDepthUtils
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

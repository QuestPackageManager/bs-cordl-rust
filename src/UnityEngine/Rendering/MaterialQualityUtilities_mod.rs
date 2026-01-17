#[cfg(feature = "cordl_class_UnityEngine+Rendering+MaterialQualityUtilities")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct MaterialQualityUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+MaterialQualityUtilities")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::MaterialQualityUtilities
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "MaterialQualityUtilities";
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
#[cfg(feature = "UnityEngine+Rendering+MaterialQualityUtilities")]
impl std::ops::Deref for crate::UnityEngine::Rendering::MaterialQualityUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+MaterialQualityUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::MaterialQualityUtilities {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+MaterialQualityUtilities")]
impl crate::UnityEngine::Rendering::MaterialQualityUtilities {
    pub fn FromIndex(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::MaterialQuality> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::Rendering::MaterialQuality,
                        1usize,
                    >("FromIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::MaterialQuality =
            unsafe { cordl_method_info.invoke_unchecked((), (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetClosestQuality(
        availableLevels: crate::UnityEngine::Rendering::MaterialQuality,
        requestedLevel: crate::UnityEngine::Rendering::MaterialQuality,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::MaterialQuality> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::MaterialQuality,
                        crate::UnityEngine::Rendering::MaterialQuality,
                    ), crate::UnityEngine::Rendering::MaterialQuality, 2usize>(
                        "GetClosestQuality"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetClosestQuality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::MaterialQuality =
            unsafe { cordl_method_info.invoke_unchecked((), (availableLevels, requestedLevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHighestQuality(
        levels: crate::UnityEngine::Rendering::MaterialQuality,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::MaterialQuality> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::MaterialQuality),
                        crate::UnityEngine::Rendering::MaterialQuality,
                        1usize,
                    >("GetHighestQuality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHighestQuality", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::MaterialQuality =
            unsafe { cordl_method_info.invoke_unchecked((), (levels))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalShaderKeywords_CommandBuffer1(
        level: crate::UnityEngine::Rendering::MaterialQuality,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::MaterialQuality,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalShaderKeywords"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalShaderKeywords",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (level, cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalShaderKeywords_MaterialQuality0(
        level: crate::UnityEngine::Rendering::MaterialQuality,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::MaterialQuality),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetGlobalShaderKeywords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalShaderKeywords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (level))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToFirstIndex(
        level: crate::UnityEngine::Rendering::MaterialQuality,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::MaterialQuality),
                        i32,
                        1usize,
                    >("ToFirstIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToFirstIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (level))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+MaterialQualityUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::MaterialQualityUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

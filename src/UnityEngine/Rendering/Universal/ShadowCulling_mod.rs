#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowCulling")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ShadowCulling {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowCulling")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ShadowCulling
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShadowCulling";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowCulling")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ShadowCulling {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowCulling")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ShadowCulling {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShadowCulling")]
impl crate::UnityEngine::Rendering::Universal::ShadowCulling {
    pub fn ComputeShadowCasterCullingInfos(
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        shadowAtlasLayout: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout,
        >,
        cullingResults: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CullingResults,
        >,
        shadowCullingInfos: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ShadowCastersCullingInfos,
        >,
        urpVisibleLightsShadowCullingInfos: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<
                crate::UnityEngine::Rendering::Universal::URPLightShadowCullingInfos,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ShadowCastersCullingInfos,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeArray_1<
                                    crate::UnityEngine::Rendering::Universal::URPLightShadowCullingInfos,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ComputeShadowCasterCullingInfos")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeShadowCasterCullingInfos", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    shadowData,
                    shadowAtlasLayout,
                    cullingResults,
                    shadowCullingInfos,
                    urpVisibleLightsShadowCullingInfos,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CullShadowCasters(
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
        shadowAtlasLayout: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout,
        >,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::Universal::URPLightShadowCullingInfos,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::ScriptableRenderContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::AdditionalLightsShadowAtlasLayout,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                        ),
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::Universal::URPLightShadowCullingInfos,
                        >,
                        4usize,
                    >("CullShadowCasters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CullShadowCasters", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::Universal::URPLightShadowCullingInfos,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (context, shadowData, shadowAtlasLayout, cullResults))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCullingProjectionType(
        _cordl_type: crate::UnityEngine::LightType,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::BatchCullingProjectionType>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::LightType),
                        crate::UnityEngine::Rendering::BatchCullingProjectionType,
                        1usize,
                    >("GetCullingProjectionType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCullingProjectionType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::BatchCullingProjectionType =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShadowCulling")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Universal::ShadowCulling {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderPipelineGlobalSettings_2<
    TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
    TRenderPipeline: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::Rendering::RenderPipelineGlobalSettings,
    __cordl_phantom_TGlobalRenderPipelineSettings:
        std::marker::PhantomData<TGlobalRenderPipelineSettings>,
    __cordl_phantom_TRenderPipeline: std::marker::PhantomData<TRenderPipeline>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
unsafe impl<
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
        TRenderPipeline: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        TGlobalRenderPipelineSettings,
        TRenderPipeline,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RenderPipelineGlobalSettings`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.Rendering",
                "RenderPipelineGlobalSettings`2",
            )
            .unwrap()
            .make_generic::<(TGlobalRenderPipelineSettings, TRenderPipeline)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
impl<
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
        TRenderPipeline: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        TGlobalRenderPipelineSettings,
        TRenderPipeline,
    >
{
    type Target = crate::UnityEngine::Rendering::RenderPipelineGlobalSettings;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
impl<
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
        TRenderPipeline: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        TGlobalRenderPipelineSettings,
        TRenderPipeline,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
impl<
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
        TRenderPipeline: quest_hook::libil2cpp::Type,
    >
    crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        TGlobalRenderPipelineSettings,
        TRenderPipeline,
    >
{
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TRenderPipeline: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TRenderPipeline: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TRenderPipeline: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<TGlobalRenderPipelineSettings>
    where
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TRenderPipeline: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), TGlobalRenderPipelineSettings, 0usize>("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_instance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TGlobalRenderPipelineSettings =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderPipelineGlobalSettings_2")]
impl<
        TGlobalRenderPipelineSettings: quest_hook::libil2cpp::Type,
        TRenderPipeline: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        TGlobalRenderPipelineSettings,
        TRenderPipeline,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

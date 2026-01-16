#[cfg(feature = "cordl_class_UnityEngine+UIElements+BackgroundPropertyHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BackgroundPropertyHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BackgroundPropertyHelper")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::BackgroundPropertyHelper
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BackgroundPropertyHelper";
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
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPropertyHelper")]
impl crate::UnityEngine::UIElements::BackgroundPropertyHelper {
    pub fn ConvertScaleModeToBackgroundPosition(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundPosition> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ScaleMode),
                        crate::UnityEngine::UIElements::BackgroundPosition,
                        1usize,
                    >("ConvertScaleModeToBackgroundPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertScaleModeToBackgroundPosition", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertScaleModeToBackgroundRepeat(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundRepeat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ScaleMode),
                        crate::UnityEngine::UIElements::BackgroundRepeat,
                        1usize,
                    >("ConvertScaleModeToBackgroundRepeat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertScaleModeToBackgroundRepeat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertScaleModeToBackgroundSize(
        scaleMode: crate::UnityEngine::ScaleMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::ScaleMode),
                        crate::UnityEngine::UIElements::BackgroundSize,
                        1usize,
                    >("ConvertScaleModeToBackgroundSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertScaleModeToBackgroundSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveUnityBackgroundScaleMode(
        backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
        backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
        backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
        backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
        valid: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ScaleMode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::UIElements::BackgroundPosition,
                        crate::UnityEngine::UIElements::BackgroundPosition,
                        crate::UnityEngine::UIElements::BackgroundRepeat,
                        crate::UnityEngine::UIElements::BackgroundSize,
                        quest_hook::libil2cpp::ByRefMut<bool>,
                    ), crate::UnityEngine::ScaleMode, 5usize>(
                        "ResolveUnityBackgroundScaleMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResolveUnityBackgroundScaleMode",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ScaleMode = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    backgroundPositionX,
                    backgroundPositionY,
                    backgroundRepeat,
                    backgroundSize,
                    valid,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BackgroundPropertyHelper")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::BackgroundPropertyHelper
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

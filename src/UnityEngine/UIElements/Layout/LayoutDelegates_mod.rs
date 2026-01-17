#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDelegates")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct LayoutDelegates {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDelegates")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::Layout::LayoutDelegates
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutDelegates";
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
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutDelegates")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Layout::LayoutDelegates {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutDelegates")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Layout::LayoutDelegates {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutDelegates")]
impl crate::UnityEngine::UIElements::Layout::LayoutDelegates {
    pub fn InvokeBaselineFunction(
        node: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Layout::LayoutNode>,
        width: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutNode,
                        >,
                        f32,
                        f32,
                    ), f32, 3usize>("InvokeBaselineFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeBaselineFunction",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked((), (node, width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMeasureFunction(
        node: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Layout::LayoutNode>,
        width: f32,
        widthMode: crate::UnityEngine::UIElements::Layout::LayoutMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::UIElements::Layout::LayoutMeasureMode,
        exception: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Layout::LayoutSize>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutNode,
                        >,
                        f32,
                        crate::UnityEngine::UIElements::Layout::LayoutMeasureMode,
                        f32,
                        crate::UnityEngine::UIElements::Layout::LayoutMeasureMode,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::Layout::LayoutSize,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "InvokeMeasureFunction"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeMeasureFunction",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    node, width, widthMode, height, heightMode, exception, result,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutDelegates")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Layout::LayoutDelegates {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

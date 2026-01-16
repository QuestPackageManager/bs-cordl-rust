#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutProcessorNative")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutProcessorNative {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_State: crate::UnityEngine::UIElements::Layout::LayoutState,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutProcessorNative")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutProcessorNative";
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
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl crate::UnityEngine::UIElements::Layout::LayoutProcessorNative {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_UIElements_Layout_ILayoutProcessor_CalculateLayout(
        &mut self,
        node: crate::UnityEngine::UIElements::Layout::LayoutNode,
        parentWidth: f32,
        parentHeight: f32,
        parentDirection: crate::UnityEngine::UIElements::Layout::LayoutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::Layout::LayoutNode,
                        f32,
                        f32,
                        crate::UnityEngine::UIElements::Layout::LayoutDirection,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "UnityEngine.UIElements.Layout.ILayoutProcessor.CalculateLayout",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.UIElements.Layout.ILayoutProcessor.CalculateLayout",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (node, parentWidth, parentHeight, parentDirection))?
        };
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
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl AsRef<crate::UnityEngine::UIElements::Layout::ILayoutProcessor>
    for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative
{
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::Layout::ILayoutProcessor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutProcessorNative")]
impl AsMut<crate::UnityEngine::UIElements::Layout::ILayoutProcessor>
    for crate::UnityEngine::UIElements::Layout::LayoutProcessorNative
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::Layout::ILayoutProcessor {
        unsafe { std::mem::transmute(self) }
    }
}

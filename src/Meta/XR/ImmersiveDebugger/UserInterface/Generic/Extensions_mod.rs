#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger.UserInterface.Generic";
    const CLASS_NAME: &'static str = "Extensions";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions")]
impl std::ops::Deref
for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions")]
impl std::ops::DerefMut
for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Extensions {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions")]
impl crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Extensions {
    pub fn SetSizeOptimized(
        rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
        offsetMin: crate::UnityEngine::Vector2,
        offsetMax: crate::UnityEngine::Vector2,
        fixedDimensions: crate::UnityEngine::Vector2,
        setAnchoredPosition: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetSizeOptimized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSizeOptimized", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        rectTransform,
                        offsetMin,
                        offsetMax,
                        fixedDimensions,
                        setAnchoredPosition,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+ImmersiveDebugger+UserInterface+Generic+Extensions"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "cordl_class_UnityEngine+UIElements+CustomBinding")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct CustomBinding {
    __cordl_parent: crate::UnityEngine::UIElements::Binding,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+CustomBinding")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::CustomBinding {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "CustomBinding";
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
#[cfg(feature = "UnityEngine+UIElements+CustomBinding")]
impl std::ops::Deref for crate::UnityEngine::UIElements::CustomBinding {
    type Target = crate::UnityEngine::UIElements::Binding;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CustomBinding")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::CustomBinding {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CustomBinding")]
impl crate::UnityEngine::UIElements::CustomBinding {
    pub fn Update(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingContext>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingResult> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::BindingContext,
                    >), crate::UnityEngine::UIElements::BindingResult, 1usize>(
                        "Update"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingResult =
            unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+CustomBinding")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::CustomBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

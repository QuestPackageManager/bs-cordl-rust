#[cfg(feature = "cordl_class_UnityEngine+UIElements+DataBindingUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct DataBindingUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+DataBindingUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::DataBindingUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "DataBindingUtility";
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
#[cfg(feature = "UnityEngine+UIElements+DataBindingUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DataBindingUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DataBindingUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DataBindingUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DataBindingUtility")]
impl crate::UnityEngine::UIElements::DataBindingUtility {
    pub fn TryGetBinding(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bindingId: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingId>,
        bindingInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::BindingId>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::BindingInfo,
                        >,
                    ), bool, 3usize>("TryGetBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetBinding",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (element, bindingId, bindingInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+DataBindingUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::DataBindingUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BindingInfo {
    pub _targetElement_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _bindingId_k__BackingField: crate::UnityEngine::UIElements::BindingId,
    pub _binding_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Binding,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::BindingInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BindingInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::BindingInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::BindingInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::BindingInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::BindingInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+BindingInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::BindingInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BindingInfo")]
impl crate::UnityEngine::UIElements::BindingInfo {
    pub fn FromBindingData(
        bindingData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::DataBindingManager_BindingData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::DataBindingManager_BindingData,
                            >,
                        >),
                        crate::UnityEngine::UIElements::BindingInfo,
                        1usize,
                    >("FromBindingData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromBindingData", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingInfo = unsafe {
            cordl_method_info.invoke_unchecked((), (bindingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromRequest(
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        targetPath: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Properties::PropertyPath,
        >,
        binding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BindingInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Properties::PropertyPath,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Binding,
                            >,
                        ),
                        crate::UnityEngine::UIElements::BindingInfo,
                        3usize,
                    >("FromRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromRequest", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::BindingInfo = unsafe {
            cordl_method_info.invoke_unchecked((), (target, targetPath, binding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        targetElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        bindingId: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BindingId,
        >,
        binding: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::UIElements::BindingId,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Binding,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (targetElement, bindingId, binding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_binding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Binding>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Binding,
                        >,
                        0usize,
                    >("get_binding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_binding", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Binding,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}

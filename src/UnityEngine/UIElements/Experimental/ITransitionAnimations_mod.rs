#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
#[repr(C)]
#[derive(Debug)]
pub struct ITransitionAnimations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Experimental";
    const CLASS_NAME: &'static str = "ITransitionAnimations";
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
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    pub fn Start(
        &mut self,
        to: crate::UnityEngine::UIElements::Experimental::StyleValues,
        durationMs: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Experimental::StyleValues, i32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                                crate::UnityEngine::UIElements::Experimental::StyleValues,
                            >,
                        >,
                        2usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Experimental::ValueAnimation_1<
                crate::UnityEngine::UIElements::Experimental::StyleValues,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (to, durationMs))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Experimental+ITransitionAnimations")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Experimental::ITransitionAnimations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

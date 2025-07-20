#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct EventInterestReflectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventInterestReflectionUtils";
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
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    #[cfg(
        feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
    )]
    pub type DefaultEventInterests = crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests;
    pub fn ComputeDefaultEventInterests(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        i32,
                        2usize,
                    >("ComputeDefaultEventInterests")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ComputeDefaultEventInterests", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (elementType, methodName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultEventInterests(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        defaultActionCategories: quest_hook::libil2cpp::ByRefMut<i32>,
        defaultActionAtTargetCategories: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("GetDefaultEventInterests")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDefaultEventInterests", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        elementType,
                        defaultActionCategories,
                        defaultActionAtTargetCategories,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventCategory(
        eventType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::EventCategory> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        crate::UnityEngine::UIElements::EventCategory,
                        1usize,
                    >("GetEventCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetEventCategory", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::EventCategory = unsafe {
            method.invoke_unchecked((), (eventType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventInterestReflectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventInterestReflectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventInterestReflectionUtils_DefaultEventInterests {
    pub DefaultActionCategories: i32,
    pub DefaultActionAtTargetCategories: i32,
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "EventInterestReflectionUtils/DefaultEventInterests";
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
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
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
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+EventInterestReflectionUtils+DefaultEventInterests"
)]
impl crate::UnityEngine::UIElements::EventInterestReflectionUtils_DefaultEventInterests {}

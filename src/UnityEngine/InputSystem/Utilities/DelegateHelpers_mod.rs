#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "DelegateHelpers";
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    pub fn InvokeCallbacksSafe_AndInvokeReturnedActions<TValue>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Func_2<
                        TValue,
                        quest_hook::libil2cpp::Gc<crate::System::Action>,
                    >,
                >,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Func_2<
                                            TValue,
                                            quest_hook::libil2cpp::Gc<crate::System::Action>,
                                        >,
                                    >,
                                >,
                            >,
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InvokeCallbacksSafe_AndInvokeReturnedActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe_AndInvokeReturnedActions", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callbacks, argument, callbackName, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_AnyCallbackReturnsObject<TValue, TReturn>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, TReturn>>,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Func_2<TValue, TReturn>,
                                    >,
                                >,
                            >,
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        4usize,
                    >("InvokeCallbacksSafe_AnyCallbackReturnsObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe_AnyCallbackReturnsObject", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (callbacks, argument, callbackName, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_AnyCallbackReturnsTrue<TValue1, TValue2>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Func_3<TValue1, TValue2, bool>>,
            >,
        >,
        argument1: TValue1,
        argument2: TValue2,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Func_3<TValue1, TValue2, bool>,
                                    >,
                                >,
                            >,
                            TValue1,
                            TValue2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        5usize,
                    >("InvokeCallbacksSafe_AnyCallbackReturnsTrue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe_AnyCallbackReturnsTrue", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (callbacks, argument1, argument2, callbackName, context),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_Il2CppString_Il2CppObject0(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Action>,
            >,
        >,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeCallbacksSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callbacks, callbackName, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_TValue1_TValue2_Il2CppString_Il2CppObject2<
        TValue1,
        TValue2,
    >(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Action_2<TValue1, TValue2>>,
            >,
        >,
        argument1: TValue1,
        argument2: TValue2,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Action_2<TValue1, TValue2>,
                                    >,
                                >,
                            >,
                            TValue1,
                            TValue2,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InvokeCallbacksSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (callbacks, argument1, argument2, callbackName, context),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCallbacksSafe_TValue_Il2CppString_Il2CppObject1<TValue>(
        callbacks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
            >,
        >,
        argument: TValue,
        callbackName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<
                                    quest_hook::libil2cpp::Gc<crate::System::Action_1<TValue>>,
                                >,
                            >,
                            TValue,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InvokeCallbacksSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCallbacksSafe", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callbacks, argument, callbackName, context))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+DelegateHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::DelegateHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

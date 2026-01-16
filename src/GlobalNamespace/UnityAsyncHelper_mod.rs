#[cfg(feature = "cordl_class_UnityAsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityAsyncHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityAsyncHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UnityAsyncHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityAsyncHelper";
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
#[cfg(feature = "UnityAsyncHelper")]
impl std::ops::Deref for crate::GlobalNamespace::UnityAsyncHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityAsyncHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityAsyncHelper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityAsyncHelper")]
impl crate::GlobalNamespace::UnityAsyncHelper {
    pub fn InvokeSafe_Func_1_0(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Func_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InvokeSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeSafe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncTask))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_Func_2_A1<A>(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                A,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
        firstParameter: A,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    A,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task,
                                    >,
                                >,
                            >,
                            A,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InvokeSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeSafe", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncTask, firstParameter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_Func_3_A_B2<A, B>(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                A,
                B,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
        firstParameter: A,
        secondParameter: B,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_3<
                                    A,
                                    B,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task,
                                    >,
                                >,
                            >,
                            A,
                            B,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeSafe", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (asyncTask, firstParameter, secondParameter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_Func_4_A_B_C3<A, B, C>(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                A,
                B,
                C,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_4<
                                    A,
                                    B,
                                    C,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task,
                                    >,
                                >,
                            >,
                            A,
                            B,
                            C,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InvokeSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeSafe", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (asyncTask, firstParameter, secondParameter, thirdParameter),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSafe_Func_5_A_B_C_D4<A, B, C, D>(
        asyncTask: quest_hook::libil2cpp::Gc<
            crate::System::Func_5<
                A,
                B,
                C,
                D,
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
            >,
        >,
        firstParameter: A,
        secondParameter: B,
        thirdParameter: C,
        fourthParameter: D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        C: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        D: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_5<
                                    A,
                                    B,
                                    C,
                                    D,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Threading::Tasks::Task,
                                    >,
                                >,
                            >,
                            A,
                            B,
                            C,
                            D,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InvokeSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeSafe", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        asyncTask,
                        firstParameter,
                        secondParameter,
                        thirdParameter,
                        fourthParameter,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilAsync_ICoroutineStarter1(
        coroutineStarter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICoroutineStarter,
        >,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ICoroutineStarter,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        2usize,
                    >("WaitUntilAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WaitUntilAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (coroutineStarter, predicate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilAsync_MonoBehaviour0(
        coroutineStarter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                            quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        2usize,
                    >("WaitUntilAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WaitUntilAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (coroutineStarter, predicate))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityAsyncHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityAsyncHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

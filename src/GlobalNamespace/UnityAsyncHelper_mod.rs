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
    pub fn InvokeSafe_A1<A>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
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
                                A,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
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
    pub fn InvokeSafe_A_B2<A, B>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
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
                                A,
                                B,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
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
    pub fn InvokeSafe_A_B_C3<A, B, C>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            C,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
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
                                A,
                                B,
                                C,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
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
    pub fn InvokeSafe_A_B_C_D4<A, B, C, D>(
        asyncTask: quest_hook::libil2cpp::Gc<
            A,
            B,
            C,
            D,
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
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
                                A,
                                B,
                                C,
                                D,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Threading::Tasks::Task,
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
    pub fn InvokeSafe_Gc0(
        asyncTask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
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
    pub fn WaitUntilAsync_ICoroutineStarter1(
        coroutineStarter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICoroutineStarter,
        >,
        predicate: quest_hook::libil2cpp::Gc<bool>,
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
                            quest_hook::libil2cpp::Gc<bool>,
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
        predicate: quest_hook::libil2cpp::Gc<bool>,
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
                            quest_hook::libil2cpp::Gc<bool>,
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
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
#[repr(C)]
#[derive(Debug)]
pub struct __c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub __1__state: i32,
    pub __2__current: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityAsyncHelper/<>c__DisplayClass5_0/<<WaitUntilAsync>g__WaitUntilPredicateTrue|0>d";
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
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::Deref
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (__1__state))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_Generic_IEnumerator_System_Object__get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >(
                        "System.Collections.Generic.IEnumerator<System.Object>.get_Current",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.Generic.IEnumerator<System.Object>.get_Current",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("System.Collections.IEnumerator.Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.Reset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("System.IDisposable.Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IDisposable.Dispose", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (__1__state))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass5_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass5_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
#[repr(C)]
#[derive(Debug)]
pub struct __c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub __1__state: i32,
    pub __2__current: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub __4__this: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityAsyncHelper/<>c__DisplayClass6_0/<<WaitUntilAsync>g__WaitUntilPredicateTrue|0>d";
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
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::Deref
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (__1__state))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_Generic_IEnumerator_System_Object__get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >(
                        "System.Collections.Generic.IEnumerator<System.Object>.get_Current",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.Generic.IEnumerator<System.Object>.get_Current",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("System.Collections.IEnumerator.Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.Reset", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("System.IDisposable.Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.IDisposable.Dispose", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        __1__state: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (__1__state))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::Collections::IEnumerator>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityAsyncHelper+__c__DisplayClass6_0+__WaitUntilAsync_g__WaitUntilPredicateTrue_0_d"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::GlobalNamespace::__c__DisplayClass6_0_UnityAsyncHelper___WaitUntilAsync_g__WaitUntilPredicateTrue_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

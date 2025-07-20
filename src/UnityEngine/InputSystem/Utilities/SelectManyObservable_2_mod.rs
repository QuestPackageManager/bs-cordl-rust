#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectManyObservable_2<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
    pub m_Filter: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            TSource,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<TResult>,
            >,
        >,
    >,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "SelectManyObservable`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.Utilities",
                        "SelectManyObservable`2",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TResult)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<TSource, TResult> {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
    pub type Select = crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
        TSource,
        TResult,
    >;
    pub fn New(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TResult>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, filter))?;
        Ok(__cordl_object.into())
    }
    pub fn Subscribe(
        &mut self,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    >
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>),
                        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                        1usize,
                    >("Subscribe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Subscribe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            cordl_method_info.invoke_unchecked(self, (observer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TSource>>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                TSource,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TResult>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::IObservable_1<TSource>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<
                                    TSource,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::IEnumerable_1<TResult>,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, filter))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IObservable_1<TResult>>
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    fn as_ref(&self) -> &crate::System::IObservable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IObservable_1<TResult>>
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
    TSource,
    TResult,
> {
    fn as_mut(&mut self) -> &mut crate::System::IObservable_1<TResult> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectManyObservable_2_Select<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Observable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
            TSource,
            TResult,
        >,
    >,
    pub m_Observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    __cordl_phantom_TSource: std::marker::PhantomData<TSource>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
unsafe impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "SelectManyObservable`2/Select";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.Utilities",
                        "SelectManyObservable`2/Select",
                    )
                    .unwrap()
                    .make_generic::<(TSource, TResult)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    pub fn New(
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
                TSource,
                TResult,
            >,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (observable, observer))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCompleted", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnError(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OnError",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNext(
        &mut self,
        evt: TSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TSource),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "OnNext",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (evt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
                TSource,
                TResult,
            >,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSource: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2<
                                    TSource,
                                    TResult,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::IObserver_1<TResult>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (observable, observer))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IObserver_1<TSource>>
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_ref(&self) -> &crate::System::IObserver_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SelectManyObservable_2+Select")]
impl<
    TSource: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IObserver_1<TSource>>
for crate::UnityEngine::InputSystem::Utilities::SelectManyObservable_2_Select<
    TSource,
    TResult,
> {
    fn as_mut(&mut self) -> &mut crate::System::IObserver_1<TSource> {
        unsafe { std::mem::transmute(self) }
    }
}

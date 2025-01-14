#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct WhereObservable_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
    pub m_Predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "WhereObservable`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.Utilities",
                        "WhereObservable`1",
                    )
                    .unwrap()
                    .make_generic::<(TValue)>()
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
    pub type Where = crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<
        TValue,
    >;
    pub fn New(
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (source, predicate))?;
        Ok(__cordl_object.into())
    }
    pub fn Subscribe(
        &mut self,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                1usize,
            >("Subscribe")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Subscribe", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked(self, (observer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IObservable_1<TValue>>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TValue, bool>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, predicate))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::IObservable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    fn as_ref(&self) -> &crate::System::IObservable_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::IObservable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::IObservable_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
#[repr(C)]
#[derive(Debug)]
pub struct WhereObservable_1_Where<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Observable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue>,
    >,
    pub m_Observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "WhereObservable`1/Where";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputSystem.Utilities",
                        "WhereObservable`1/Where",
                    )
                    .unwrap()
                    .make_generic::<(TValue)>()
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    pub fn New(
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue>,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnCompleted", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnError(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnError", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (error))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnNext(
        &mut self,
        evt: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(TValue), quest_hook::libil2cpp::Void, 1usize>("OnNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnNext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (evt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        observable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<TValue>,
        >,
        observer: quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::Utilities::WhereObservable_1<
                            TValue,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::IObserver_1<TValue>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (observable, observer))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::IObserver_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    fn as_ref(&self) -> &crate::System::IObserver_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+WhereObservable_1+Where")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::IObserver_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::WhereObservable_1_Where<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::IObserver_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_List: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<T>,
    >,
    pub m_CreateFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    pub m_ActionOnGet: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    pub m_ActionOnRelease: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    pub m_ActionOnDestroy: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    pub m_MaxSize: i32,
    pub m_CollectionCheck: bool,
    pub _CountAll_k__BackingField: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Pool";
    const CLASS_NAME: &'static str = "ObjectPool`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.Pool",
                        "ObjectPool`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
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
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::Pool::ObjectPool_1<T> {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Get_0(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), T, 0usize>("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 0usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Get_ByRefMut1(
        &mut self,
        v: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pool::PooledObject_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<T>),
                crate::UnityEngine::Pool::PooledObject_1<T>,
                1usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Pool::PooledObject_1<T> = unsafe {
            method.invoke_unchecked(self, (v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        createFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        actionOnGet: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        actionOnRelease: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        actionOnDestroy: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        collectionCheck: bool,
        defaultCapacity: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    createFunc,
                    actionOnGet,
                    actionOnRelease,
                    actionOnDestroy,
                    collectionCheck,
                    defaultCapacity,
                    maxSize,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        element: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(T), quest_hook::libil2cpp::Void, 1usize>("Release")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Release", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        createFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        actionOnGet: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        actionOnRelease: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        actionOnDestroy: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        collectionCheck: bool,
        defaultCapacity: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
                    quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
                    bool,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        createFunc,
                        actionOnGet,
                        actionOnRelease,
                        actionOnDestroy,
                        collectionCheck,
                        defaultCapacity,
                        maxSize,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CountAll(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_CountAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CountAll", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_CountInactive(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_CountInactive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CountInactive", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_CountAll(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_CountAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_CountAll", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::UnityEngine::Pool::IObjectPool_1<T>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_ref(&self) -> &crate::UnityEngine::Pool::IObjectPool_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::UnityEngine::Pool::IObjectPool_1<T>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Pool::IObjectPool_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}

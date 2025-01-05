#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_List: quest_hook::libil2cpp::Gc<T>,
    pub m_CreateFunc: quest_hook::libil2cpp::Gc<T>,
    pub m_ActionOnGet: quest_hook::libil2cpp::Gc<T>,
    pub m_ActionOnRelease: quest_hook::libil2cpp::Gc<T>,
    pub m_ActionOnDestroy: quest_hook::libil2cpp::Gc<T>,
    pub m_MaxSize: i32,
    pub m_CollectionCheck: bool,
    pub _CountAll_k__BackingField: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Pool::ObjectPool_1 < T > =>
    "UnityEngine.Pool"."ObjectPool`1" < T >
);
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_0(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Get", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pool::PooledObject_1<T> = __cordl_object
            .invoke("Get", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        createFunc: quest_hook::libil2cpp::Gc<T>,
        actionOnGet: quest_hook::libil2cpp::Gc<T>,
        actionOnRelease: quest_hook::libil2cpp::Gc<T>,
        actionOnDestroy: quest_hook::libil2cpp::Gc<T>,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        createFunc: quest_hook::libil2cpp::Gc<T>,
        actionOnGet: quest_hook::libil2cpp::Gc<T>,
        actionOnRelease: quest_hook::libil2cpp::Gc<T>,
        actionOnDestroy: quest_hook::libil2cpp::Gc<T>,
        collectionCheck: bool,
        defaultCapacity: i32,
        maxSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
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
        Ok(__cordl_ret.into())
    }
    pub fn get_CountAll(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CountAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CountInactive(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CountInactive", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CountAll", (value))?;
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
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Pool+ObjectPool_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::Pool::ObjectPool_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}

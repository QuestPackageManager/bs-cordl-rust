#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PooledObject_1<T: quest_hook::libil2cpp::Type> {
    pub m_ToReturn: T,
    pub m_Pool: quest_hook::libil2cpp::Gc<crate::UnityEngine::Pool::IObjectPool_1<T>>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Pool::PooledObject_1 < T > =>
    "UnityEngine.Pool"."PooledObject`1<T>" < T >
);
#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Pool::PooledObject_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::Pool::PooledObject_1<T> {
    pub fn System_IDisposable_Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IDisposable.Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        value: T,
        pool: quest_hook::libil2cpp::Gc<crate::UnityEngine::Pool::IObjectPool_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, pool),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::Pool::PooledObject_1<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Pool+PooledObject_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::Pool::PooledObject_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}

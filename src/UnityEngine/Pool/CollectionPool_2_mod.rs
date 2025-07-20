#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionPool_2<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TCollection: std::marker::PhantomData<TCollection>,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
unsafe impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Pool";
    const CLASS_NAME: &'static str = "CollectionPool`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.Pool",
                        "CollectionPool`2",
                    )
                    .unwrap()
                    .make_generic::<(TCollection, TItem)>()
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
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    pub fn Get_0() -> quest_hook::libil2cpp::Result<TCollection>
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(), TCollection, 0usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Get", 0usize
                        )
                    })
            });
        let __cordl_ret: TCollection = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Get_ByRefMut1(
        value: quest_hook::libil2cpp::ByRefMut<TCollection>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Pool::PooledObject_1<TCollection>,
    >
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<TCollection>),
                        crate::UnityEngine::Pool::PooledObject_1<TCollection>,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Get", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Pool::PooledObject_1<TCollection> = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        toRelease: TCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TCollection: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (TCollection),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Release", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (toRelease))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Pool+CollectionPool_2")]
impl<
    TCollection: quest_hook::libil2cpp::Type,
    TItem: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Pool::CollectionPool_2<TCollection, TItem> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LRUCache_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    pub entryLimit: i32,
    pub cache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            TKey,
            crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue>,
        >,
    >,
    pub lru: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedList_1<TKey>,
    >,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::LRUCache_2 < TKey, TValue > =>
    "UnityEngine.ResourceManagement.Util"."LRUCache`2<TKey,TValue>" < TKey, TValue >
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2")]
unsafe impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::LRUCache_2<TKey, TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::Util::LRUCache_2<TKey, TValue> {
    #[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
    pub type Entry = crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<
        TKey,
        TValue,
    >;
    pub fn TryAdd(
        &mut self,
        id: TKey,
        obj: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryAdd",
            (id, obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGet(
        &mut self,
        offset: TKey,
        val: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGet",
            (offset, val),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (limit),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LRUCache_2_Entry<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    pub lruNode: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<TKey>,
    >,
    pub Value: TValue,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry < TKey, TValue > =>
    "UnityEngine.ResourceManagement.Util"."LRUCache`2/Entry<TKey,TValue>" < TKey, TValue
    >
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
unsafe impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue> {
    pub fn Equals(
        &mut self,
        other: crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<
            TKey,
            TValue,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue>,
    >,
> for crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+LRUCache_2+Entry")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue>,
    >,
> for crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::ResourceManagement::Util::LRUCache_2_Entry<TKey, TValue>,
    > {
        todo!()
    }
}

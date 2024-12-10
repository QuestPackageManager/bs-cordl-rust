#[cfg(feature = "OVRObjectPool")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRObjectPool {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRObjectPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool => ""
    ."OVRObjectPool"
);
#[cfg(feature = "OVRObjectPool")]
impl std::ops::Deref for crate::GlobalNamespace::OVRObjectPool {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRObjectPool")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRObjectPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRObjectPool")]
impl crate::GlobalNamespace::OVRObjectPool {
    #[cfg(feature = "OVRObjectPool+DictionaryScope_2")]
    pub type DictionaryScope_2<
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRObjectPool_DictionaryScope_2<TKey, TValue>;
    #[cfg(feature = "OVRObjectPool+HashSetScope_1")]
    pub type HashSetScope_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_HashSetScope_1<
        T,
    >;
    #[cfg(feature = "OVRObjectPool+ItemScope_1")]
    pub type ItemScope_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_ItemScope_1<
        T,
    >;
    #[cfg(feature = "OVRObjectPool+ListScope_1")]
    pub type ListScope_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_ListScope_1<
        T,
    >;
    #[cfg(feature = "OVRObjectPool+QueueScope_1")]
    pub type QueueScope_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_QueueScope_1<
        T,
    >;
    #[cfg(feature = "OVRObjectPool+StackScope_1")]
    pub type StackScope_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_StackScope_1<
        T,
    >;
    #[cfg(feature = "OVRObjectPool+Storage_1")]
    pub type Storage_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::OVRObjectPool_Storage_1<
        T,
    >;
}
#[cfg(feature = "OVRObjectPool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRObjectPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRObjectPool+DictionaryScope_2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_DictionaryScope_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    pub _dictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        TKey,
        TValue,
    >,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "OVRObjectPool+DictionaryScope_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_DictionaryScope_2
    < TKey, TValue > => ""."OVRObjectPool/DictionaryScope`2<TKey,TValue>" < TKey, TValue
    >
);
#[cfg(feature = "OVRObjectPool+DictionaryScope_2")]
unsafe impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_DictionaryScope_2<TKey, TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+DictionaryScope_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_DictionaryScope_2<TKey, TValue> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dictionary: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
        >,
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
            (dictionary),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+HashSetScope_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_HashSetScope_1<T: quest_hook::libil2cpp::Type> {
    pub _set: *mut crate::System::Collections::Generic::HashSet_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+HashSetScope_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_HashSetScope_1 <
    T > => ""."OVRObjectPool/HashSetScope`1<T>" < T >
);
#[cfg(feature = "OVRObjectPool+HashSetScope_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_HashSetScope_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+HashSetScope_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_HashSetScope_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        set: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::HashSet_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (set),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+ItemScope_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_ItemScope_1<T: quest_hook::libil2cpp::Type> {
    pub _item: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+ItemScope_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_ItemScope_1 < T >
    => ""."OVRObjectPool/ItemScope`1<T>" < T >
);
#[cfg(feature = "OVRObjectPool+ItemScope_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_ItemScope_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+ItemScope_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_ItemScope_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        item: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (item),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+ListScope_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_ListScope_1<T: quest_hook::libil2cpp::Type> {
    pub _list: *mut crate::System::Collections::Generic::List_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+ListScope_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_ListScope_1 < T >
    => ""."OVRObjectPool/ListScope`1<T>" < T >
);
#[cfg(feature = "OVRObjectPool+ListScope_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_ListScope_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+ListScope_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_ListScope_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (list),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+QueueScope_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_QueueScope_1<T: quest_hook::libil2cpp::Type> {
    pub _queue: *mut crate::System::Collections::Generic::Queue_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+QueueScope_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_QueueScope_1 < T
    > => ""."OVRObjectPool/QueueScope`1<T>" < T >
);
#[cfg(feature = "OVRObjectPool+QueueScope_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_QueueScope_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+QueueScope_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_QueueScope_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        queue: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::Queue_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (queue),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+StackScope_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRObjectPool_StackScope_1<T: quest_hook::libil2cpp::Type> {
    pub _stack: *mut crate::System::Collections::Generic::Stack_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+StackScope_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_StackScope_1 < T
    > => ""."OVRObjectPool/StackScope`1<T>" < T >
);
#[cfg(feature = "OVRObjectPool+StackScope_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRObjectPool_StackScope_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRObjectPool+StackScope_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::OVRObjectPool_StackScope_1<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        stack: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::Stack_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (stack),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRObjectPool+Storage_1")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRObjectPool_Storage_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "OVRObjectPool+Storage_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRObjectPool_Storage_1 < T >
    => ""."OVRObjectPool/Storage`1" < T >
);
#[cfg(feature = "OVRObjectPool+Storage_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::OVRObjectPool_Storage_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRObjectPool+Storage_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::OVRObjectPool_Storage_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRObjectPool+Storage_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::OVRObjectPool_Storage_1<T> {}
#[cfg(feature = "OVRObjectPool+Storage_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRObjectPool_Storage_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

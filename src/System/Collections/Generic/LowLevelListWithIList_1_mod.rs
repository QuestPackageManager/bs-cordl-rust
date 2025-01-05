#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
#[repr(C)]
#[derive(Debug)]
pub struct LowLevelListWithIList_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::LowLevelListWithIList_1 < T > =>
    "System.Collections.Generic"."LowLevelListWithIList`1" < T >
);
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    type Target = quest_hook::libil2cpp::Gc<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    #[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
    pub type Enumerator = crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<
        T,
    >;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_Generic_ICollection_T__get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.Generic.ICollection<T>.get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_T__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<T> = __cordl_object
            .invoke("System.Collections.Generic.IEnumerable<T>.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
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
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::Generic::LowLevelListWithIList_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LowLevelListWithIList_1_Enumerator<T: quest_hook::libil2cpp::Type> {
    pub _list: quest_hook::libil2cpp::Gc<T>,
    pub _index: i32,
    pub _version: i32,
    pub _current: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::LowLevelListWithIList_1_Enumerator < T > =>
    "System.Collections.Generic"."LowLevelListWithIList`1/Enumerator<T>" < T >
);
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
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
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNextRare(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNextRare",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        list: quest_hook::libil2cpp::Gc<T>,
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
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LowLevelListWithIList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::System::Collections::Generic::LowLevelListWithIList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        todo!()
    }
}

#[cfg(feature = "System+Collections+Generic+LargeArrayBuilder_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LargeArrayBuilder_1<T: quest_hook::libil2cpp::Type> {
    pub _maxCapacity: i32,
    pub _first: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    pub _buffers: crate::System::Collections::Generic::ArrayBuilder_1<
        *mut quest_hook::libil2cpp::Il2CppArray<T>,
    >,
    pub _current: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    pub _index: i32,
    pub _count: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+LargeArrayBuilder_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::LargeArrayBuilder_1 < T > =>
    "System.Collections.Generic"."LargeArrayBuilder`1<T>" < T >
);
#[cfg(feature = "System+Collections+Generic+LargeArrayBuilder_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Generic::LargeArrayBuilder_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Generic+LargeArrayBuilder_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::LargeArrayBuilder_1<T> {
    pub fn AllocateBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocateBuffer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetBuffer(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBuffer",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToArray",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        initialize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (initialize),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        maxCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (maxCapacity),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<T>,
        arrayIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (array, arrayIndex, count),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddRange(
        &mut self,
        items: *mut crate::System::Collections::Generic::IEnumerable_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddRange",
            (items),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AddWithBufferAllocation(
        &mut self,
        item: T,
        destination: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddWithBufferAllocation",
            (item, destination, index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryMove(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryMove",
            (array),
        )?;
        Ok(__cordl_ret)
    }
}

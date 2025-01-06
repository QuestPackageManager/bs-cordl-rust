#[cfg(feature = "Unity+Burst+SharedStatic_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SharedStatic_1<T: quest_hook::libil2cpp::Type> {
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::SharedStatic_1 < T > =>
    "Unity.Burst"."SharedStatic`1<T>" < T >
);
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::SharedStatic_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Burst::SharedStatic_1<T> {
    pub const DefaultAlignment: u32 = 67239952u32;
    pub fn CheckIf_T_IsUnmanagedOrThrow() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIf_T_IsUnmanagedOrThrow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreatePartiallyUnsafeWithHashCode<TSubContext>(
        alignment: u32,
        hashCode: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSubContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreatePartiallyUnsafeWithHashCode", (alignment, hashCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreatePartiallyUnsafeWithSubHashCode<TContext>(
        alignment: u32,
        subHashCode: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetOrCreatePartiallyUnsafeWithSubHashCode",
                (alignment, subHashCode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateUnsafe(
        alignment: u32,
        hashCode: i64,
        subHashCode: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateUnsafe", (alignment, hashCode, subHashCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreate_Type_Type_u32_3(
        contextType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        subContextType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreate", (contextType, subContextType, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreate_Type_u32_2(
        contextType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreate", (contextType, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreate_u32_0<TContext>(
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreate", (alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreate_u32_1<TContext, TSubContext>(
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::SharedStatic_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TSubContext: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::SharedStatic_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreate", (alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Data(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Data",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnsafeDataPointer(
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
            "get_UnsafeDataPointer",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}

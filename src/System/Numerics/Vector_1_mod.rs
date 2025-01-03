#[cfg(feature = "System+Numerics+Vector_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector_1<T: quest_hook::libil2cpp::Type> {
    pub _cordl_register: crate::System::Numerics::Register,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Numerics+Vector_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::Vector_1 < T > =>
    "System.Numerics"."Vector`1<T>" < T >
);
#[cfg(feature = "System+Numerics+Vector_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Numerics::Vector_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+Vector_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Numerics::Vector_1<T> {
    #[cfg(feature = "System+Numerics+Vector_1+VectorSizeHelper")]
    pub type VectorSizeHelper = crate::System::Numerics::Vector_1_VectorSizeHelper<T>;
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vector_1_1(
        &mut self,
        other: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vector_1_Vector_1_2(
        left: crate::System::Numerics::Vector_1<T>,
        right: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::Vector_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Numerics::Vector_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Equals", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllBitsSetValue() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllBitsSetValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOneValue() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOneValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeCount() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScalarEquals(left: T, right: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScalarEquals", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut3(
        &mut self,
        existingRegister: quest_hook::libil2cpp::ByRefMut<
            crate::System::Numerics::Register,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (existingRegister),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject1(
        &mut self,
        dataPointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dataPointer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppObject_i32_2(
        &mut self,
        dataPointer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dataPointer, offset),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_T0(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Zero() -> quest_hook::libil2cpp::Result<
        crate::System::Numerics::Vector_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Numerics::Vector_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Numerics::Vector_1<T>,
        right: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit(
        value: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::Vector_1<u64>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Numerics::Vector_1<u64> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: crate::System::Numerics::Vector_1<T>,
        right: crate::System::Numerics::Vector_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+Vector_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IEquatable_1<crate::System::Numerics::Vector_1<T>>>
for crate::System::Numerics::Vector_1<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::System::Numerics::Vector_1<T>> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+Vector_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IEquatable_1<crate::System::Numerics::Vector_1<T>>>
for crate::System::Numerics::Vector_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::Numerics::Vector_1<T>> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+Vector_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IFormattable>
for crate::System::Numerics::Vector_1<T> {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+Vector_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IFormattable>
for crate::System::Numerics::Vector_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+Vector_1+VectorSizeHelper")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector_1_VectorSizeHelper<T: quest_hook::libil2cpp::Type> {
    pub _placeholder: crate::System::Numerics::Vector_1<T>,
    pub _byte: u8,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Numerics+Vector_1+VectorSizeHelper")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::Vector_1_VectorSizeHelper < T
    > => "System.Numerics"."Vector`1/VectorSizeHelper<T>" < T >
);
#[cfg(feature = "System+Numerics+Vector_1+VectorSizeHelper")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Numerics::Vector_1_VectorSizeHelper<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+Vector_1+VectorSizeHelper")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Numerics::Vector_1_VectorSizeHelper<T> {}

#[cfg(feature = "Unity+Burst+SharedStatic_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SharedStatic_1<T: quest_hook::libil2cpp::Type> {
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Burst::SharedStatic_1<T> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst";
    const CLASS_NAME: &'static str = "SharedStatic`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("Unity.Burst", "SharedStatic`1")
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::Unity::Burst::SharedStatic_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::SharedStatic_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::Unity::Burst::SharedStatic_1<T> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Unity+Burst+SharedStatic_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::Unity::Burst::SharedStatic_1<T> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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

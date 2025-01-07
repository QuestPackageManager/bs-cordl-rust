#[cfg(feature = "System+ValueTuple_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueTuple_1<T1: quest_hook::libil2cpp::Type> {
    pub Item1: T1,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::ValueTuple_1<T1> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ValueTuple`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("System", "ValueTuple`1")
                    .unwrap()
                    .make_generic::<(T1)>()
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
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::System::ValueTuple_1<T1> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::System::ValueTuple_1<T1> {
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
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::System::ValueTuple_1<T1> {
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
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::System::ValueTuple_1<T1> {
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
#[cfg(feature = "System+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::ValueTuple_1<T1> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> crate::System::ValueTuple_1<T1> {
    pub fn CompareTo(
        &mut self,
        other: crate::System::ValueTuple_1<T1>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ValueTuple_1_1(
        &mut self,
        other: crate::System::ValueTuple_1<T1>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralComparable_CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IStructuralComparable.CompareTo",
            (other, comparer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IStructuralEquatable.Equals",
            (other, comparer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_GetHashCode(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IStructuralEquatable.GetHashCode",
            (comparer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IComparable_CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IComparable.CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IValueTupleInternal_GetHashCode(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IValueTupleInternal.GetHashCode",
            (comparer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IValueTupleInternal_ToStringEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IValueTupleInternal.ToStringEnd",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_CompilerServices_ITuple_get_Length(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.CompilerServices.ITuple.get_Length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        item1: T1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (item1),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IStructuralComparable>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IStructuralComparable>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IStructuralEquatable>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralEquatable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IStructuralEquatable>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralEquatable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> AsRef<crate::System::IComparable>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> AsMut<crate::System::IComparable>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IComparable_1<crate::System::ValueTuple_1<T1>>>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::ValueTuple_1<T1>> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IComparable_1<crate::System::ValueTuple_1<T1>>>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::System::ValueTuple_1<T1>> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IEquatable_1<crate::System::ValueTuple_1<T1>>>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::ValueTuple_1<T1>> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IEquatable_1<crate::System::ValueTuple_1<T1>>>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::ValueTuple_1<T1>> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> AsRef<crate::System::IValueTupleInternal>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::IValueTupleInternal {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> AsMut<crate::System::IValueTupleInternal>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(&mut self) -> &mut crate::System::IValueTupleInternal {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ValueTuple_1<T1> {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::ITuple {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple_1")]
impl<
    T1: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ValueTuple_1<T1> {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::CompilerServices::ITuple {
        todo!()
    }
}

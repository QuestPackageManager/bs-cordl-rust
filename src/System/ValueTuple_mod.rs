#[cfg(feature = "System+ValueTuple")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueTuple {}
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ValueTuple {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ValueTuple";
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
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::ValueTuple {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::ValueTuple {
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
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::ValueTuple {
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
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::ValueTuple {
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
#[cfg(feature = "System+ValueTuple")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::ValueTuple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ValueTuple")]
impl crate::System::ValueTuple {
    pub fn CombineHashCodes_i32_1(
        h1: i32,
        h2: i32,
        h3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i32, 3usize>("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2, h3))? };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_0(
        h1: i32,
        h2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2))? };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_2(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32), i32, 4usize>("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (h1, h2, h3, h4))? };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_i32_3(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32),
                i32,
                5usize,
            >("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_i32_i32_4(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32),
                i32,
                6usize,
            >("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 6usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5, h6))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_i32_i32_i32_5(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
        h7: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32, i32),
                i32,
                7usize,
            >("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 7usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5, h6, h7))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_i32_i32_i32_i32_6(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
        h7: i32,
        h8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32, i32, i32),
                i32,
                8usize,
            >("CombineHashCodes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CombineHashCodes", 8usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (h1, h2, h3, h4, h5, h6, h7, h8))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo(
        &mut self,
        other: crate::System::ValueTuple,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::ValueTuple), i32, 1usize>("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ValueTuple1(
        &mut self,
        other: crate::System::ValueTuple,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::ValueTuple), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralComparable_CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::IComparer>,
                ),
                i32,
                2usize,
            >("System.Collections.IStructuralComparable.CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.Collections.IStructuralComparable.CompareTo",
                    2usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (other, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::IEqualityComparer,
                    >,
                ),
                bool,
                2usize,
            >("System.Collections.IStructuralEquatable.Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.Collections.IStructuralEquatable.Equals", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (other, comparer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IStructuralEquatable_GetHashCode(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::IEqualityComparer,
                >),
                i32,
                1usize,
            >("System.Collections.IStructuralEquatable.GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.Collections.IStructuralEquatable.GetHashCode",
                    1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (comparer))? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IComparable_CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i32,
                1usize,
            >("System.IComparable.CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.IComparable.CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IValueTupleInternal_GetHashCode(
        &mut self,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEqualityComparer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::IEqualityComparer,
                >),
                i32,
                1usize,
            >("System.IValueTupleInternal.GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.IValueTupleInternal.GetHashCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (comparer))? };
        Ok(__cordl_ret.into())
    }
    pub fn System_IValueTupleInternal_ToStringEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("System.IValueTupleInternal.ToStringEnd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.IValueTupleInternal.ToStringEnd", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_CompilerServices_ITuple_get_Length(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                i32,
                0usize,
            >("System.Runtime.CompilerServices.ITuple.get_Length")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "System.Runtime.CompilerServices.ITuple.get_Length",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::ValueTuple as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::ValueTuple as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::Collections::IStructuralComparable>
for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::Collections::IStructuralComparable>
for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::Collections::IStructuralEquatable>
for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::Collections::IStructuralEquatable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::Collections::IStructuralEquatable>
for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IStructuralEquatable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::IComparable> for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::IComparable> for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::IComparable_1<crate::System::ValueTuple>>
for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::ValueTuple> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::IComparable_1<crate::System::ValueTuple>>
for crate::System::ValueTuple {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::System::ValueTuple> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::IEquatable_1<crate::System::ValueTuple>>
for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::ValueTuple> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::IEquatable_1<crate::System::ValueTuple>>
for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::System::ValueTuple> {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::IValueTupleInternal> for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::IValueTupleInternal {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::IValueTupleInternal> for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::IValueTupleInternal {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsRef<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ValueTuple {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::ITuple {
        todo!()
    }
}
#[cfg(feature = "System+ValueTuple")]
impl AsMut<crate::System::Runtime::CompilerServices::ITuple>
for crate::System::ValueTuple {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::CompilerServices::ITuple {
        todo!()
    }
}

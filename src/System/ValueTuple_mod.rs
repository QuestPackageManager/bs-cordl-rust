#[cfg(feature = "System+ValueTuple")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueTuple {}
#[cfg(feature = "System+ValueTuple")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ValueTuple => "System"."ValueTuple"
);
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
    pub fn CompareTo(
        &mut self,
        other: crate::System::ValueTuple,
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ValueTuple1(
        &mut self,
        other: crate::System::ValueTuple,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    ) -> quest_hook::libil2cpp::Result<bool> {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    > {
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
    ) -> quest_hook::libil2cpp::Result<i32> {
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
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
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

#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReadOnlyArray_1<TValue: quest_hook::libil2cpp::Type> {
    pub m_Array: *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
    pub m_StartIndex: i32,
    pub m_Length: i32,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1 < TValue > =>
    "UnityEngine.InputSystem.Utilities"."ReadOnlyArray`1<TValue>" < TValue >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<
        TValue,
    >;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<
            TValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Predicate_1<TValue>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (predicate),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_TValue__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TValue>,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TValue>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<TValue>.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TValue>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_1(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array, index, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyList_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyList_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyList_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReadOnlyArray_1_Enumerator<TValue: quest_hook::libil2cpp::Type> {
    pub m_Array: *mut quest_hook::libil2cpp::Il2CppArray<TValue>,
    pub m_IndexStart: i32,
    pub m_IndexEnd: i32,
    pub m_Index: i32,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator < TValue > =>
    "UnityEngine.InputSystem.Utilities"."ReadOnlyArray`1/Enumerator<TValue>" < TValue >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
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
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
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
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
        index: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (array, index, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArray_1+Enumerator")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1_Enumerator<TValue> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}

#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneOrMore_2<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> {
    pub m_IsSingle: bool,
    pub m_Single: TValue,
    pub m_Multiple: TList,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::OneOrMore_2
    < TValue, TList > => "UnityEngine.InputSystem.Utilities"."OneOrMore`2<TValue,TList>"
    < TValue, TList >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
unsafe impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    #[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
    pub type Enumerator = crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<
        TValue,
        TList,
    >;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TValue>,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TValue>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetEnumerator", ())?;
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
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn _ctor_TList1(
        &mut self,
        multiple: TList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiple),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TValue0(
        &mut self,
        single: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (single),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_TList1(
        multiple: TList,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<
            TValue,
            TList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (multiple))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_TValue0(
        single: TValue,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<
            TValue,
            TList,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (single))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyList_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyList_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyList_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyList_1<TValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct OneOrMore_2_Enumerator<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Index: i32,
    pub m_List: crate::UnityEngine::InputSystem::Utilities::OneOrMore_2<TValue, TList>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator < TValue, TList > =>
    "UnityEngine.InputSystem.Utilities"."OneOrMore`2/Enumerator" < TValue, TList >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("get_Current", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<TValue>>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+OneOrMore_2+Enumerator")]
impl<
    TValue: quest_hook::libil2cpp::Type,
    TList: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::Utilities::OneOrMore_2_Enumerator<TValue, TList> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

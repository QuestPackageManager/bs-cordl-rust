#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UQueryState_1<T: quest_hook::libil2cpp::Type> {
    pub m_Element: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_Matchers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::RuleMatcher,
        >,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQueryState_1 < T > =>
    "UnityEngine.UIElements"."UQueryState`1<T>" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<T: quest_hook::libil2cpp::Type> crate::UnityEngine::UIElements::UQueryState_1<T> {
    #[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
    pub type ActionQueryMatcher = crate::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
    pub type Enumerator = crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T>;
    #[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
    pub type ListQueryMatcher_1<TElement: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1<
        T,
        TElement,
    >;
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_UQueryState_1_0(
        &mut self,
        other: crate::UnityEngine::UIElements::UQueryState_1<T>,
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
    pub fn First(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "First",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
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
    pub fn RebuildOn(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UQueryState_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::UQueryState_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RebuildOn",
            (element),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Single(
        &mut self,
        matcher: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Single",
            (matcher),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_T__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<T>.GetEnumerator",
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn _ctor(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        matchers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::RuleMatcher,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (element, matchers),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::UQueryState_1<T>>>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::UQueryState_1<T>> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::UQueryState_1<T>>>
for crate::UnityEngine::UIElements::UQueryState_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::UQueryState_1<T>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct UQueryState_1_ActionQueryMatcher<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::UQuery_UQueryMatcher,
    pub _callBack_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher < T > =>
    "UnityEngine.UIElements"."UQueryState`1/ActionQueryMatcher" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher<T> {
    type Target = crate::UnityEngine::UIElements::UQuery_UQueryMatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
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
    pub fn OnRuleMatchedElement(
        &mut self,
        matcher: crate::UnityEngine::UIElements::RuleMatcher,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OnRuleMatchedElement", (matcher, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
    pub fn get_callBack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>> = __cordl_object
            .invoke("get_callBack", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ActionQueryMatcher")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQueryState_1_ActionQueryMatcher<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UQueryState_1_Enumerator<T: quest_hook::libil2cpp::Type> {
    pub iterationList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub currentIndex: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQueryState_1_Enumerator < T > => "UnityEngine.UIElements"
    ."UQueryState`1/Enumerator<T>" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
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
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        queryState: crate::UnityEngine::UIElements::UQueryState_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (queryState),
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
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UQueryState_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
#[repr(C)]
#[derive(Debug)]
pub struct UQueryState_1_ListQueryMatcher_1<
    T: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::UQuery_UQueryMatcher,
    pub _matches_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<TElement>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1 < T, TElement > =>
    "UnityEngine.UIElements"."UQueryState`1/ListQueryMatcher`1" < T, TElement >
);
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
impl<
    T: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1<T, TElement> {
    type Target = crate::UnityEngine::UIElements::UQuery_UQueryMatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
impl<
    T: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1<T, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
impl<
    T: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1<T, TElement> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnRuleMatchedElement(
        &mut self,
        matcher: crate::UnityEngine::UIElements::RuleMatcher,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OnRuleMatchedElement", (matcher, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_matches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TElement>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TElement>,
        > = __cordl_object.invoke("get_matches", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_matches(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TElement>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_matches", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryState_1+ListQueryMatcher_1")]
impl<
    T: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQueryState_1_ListQueryMatcher_1<T, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

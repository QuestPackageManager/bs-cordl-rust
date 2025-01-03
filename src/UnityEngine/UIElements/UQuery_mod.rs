#[cfg(feature = "UnityEngine+UIElements+UQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQuery =>
    "UnityEngine.UIElements"."UQuery"
);
#[cfg(feature = "UnityEngine+UIElements+UQuery")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQuery {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery")]
impl crate::UnityEngine::UIElements::UQuery {
    #[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
    pub type FirstQueryMatcher = crate::UnityEngine::UIElements::UQuery_FirstQueryMatcher;
    #[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
    type IVisualPredicateWrapper = crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper;
    #[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
    pub type IsOfType_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::UQuery_IsOfType_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
    pub type SingleQueryMatcher = crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher;
    #[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
    pub type UQueryMatcher = crate::UnityEngine::UIElements::UQuery_UQueryMatcher;
}
#[cfg(feature = "UnityEngine+UIElements+UQuery")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UQuery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery_FirstQueryMatcher {
    __cordl_parent: crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQuery_FirstQueryMatcher => "UnityEngine.UIElements"
    ."UQuery/FirstQueryMatcher"
);
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQuery_FirstQueryMatcher {
    type Target = crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQuery_FirstQueryMatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
impl crate::UnityEngine::UIElements::UQuery_FirstQueryMatcher {
    pub fn CreateNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
        > = __cordl_object.invoke("CreateNew", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OnRuleMatchedElement", (matcher, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+FirstQueryMatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQuery_FirstQueryMatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery_IVisualPredicateWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper => "UnityEngine.UIElements"
    ."UQuery/IVisualPredicateWrapper"
);
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
impl crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
    pub fn Predicate(
        &mut self,
        e: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Predicate", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IVisualPredicateWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery_IsOfType_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQuery_IsOfType_1 < T >
    => "UnityEngine.UIElements"."UQuery/IsOfType`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
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
    pub fn Predicate(
        &mut self,
        e: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Predicate", (e))?;
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
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper>
for crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+IsOfType_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper>
for crate::UnityEngine::UIElements::UQuery_IsOfType_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::UQuery_IVisualPredicateWrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery_SingleQueryMatcher {
    __cordl_parent: crate::UnityEngine::UIElements::UQuery_UQueryMatcher,
    pub _match_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UQuery_SingleQueryMatcher => "UnityEngine.UIElements"
    ."UQuery/SingleQueryMatcher"
);
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher {
    type Target = crate::UnityEngine::UIElements::UQuery_UQueryMatcher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
impl crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher {
    pub fn CreateNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher,
        > = __cordl_object.invoke("CreateNew", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInUse(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInUse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        matchers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::RuleMatcher,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (root, matchers))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_match(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_match", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_match(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_match", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+SingleQueryMatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQuery_SingleQueryMatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct UQuery_UQueryMatcher {
    __cordl_parent: crate::UnityEngine::UIElements::StyleSheets::HierarchyTraversal,
    pub m_Matchers: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::RuleMatcher,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQuery_UQueryMatcher =>
    "UnityEngine.UIElements"."UQuery/UQueryMatcher"
);
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQuery_UQueryMatcher {
    type Target = crate::UnityEngine::UIElements::StyleSheets::HierarchyTraversal;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQuery_UQueryMatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
impl crate::UnityEngine::UIElements::UQuery_UQueryMatcher {
    #[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher+__c")]
    pub type __c = crate::UnityEngine::UIElements::UQueryMatcher_UQuery___c;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NoProcessResult(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        i: crate::UnityEngine::UIElements::StyleSheets::MatchResultInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NoProcessResult", (e, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRuleMatchedElement(
        &mut self,
        matcher: crate::UnityEngine::UIElements::RuleMatcher,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("OnRuleMatchedElement", (matcher, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        matchers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::RuleMatcher,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (root, matchers))?;
        Ok(__cordl_ret.into())
    }
    pub fn Traverse(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Traverse", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn TraverseRecursive(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TraverseRecursive", (element, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQuery+UQueryMatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQuery_UQueryMatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

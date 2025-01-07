#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigateFocusRing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    pub m_Ring: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElementFocusRing,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::NavigateFocusRing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "NavigateFocusRing";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::NavigateFocusRing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::NavigateFocusRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl crate::UnityEngine::UIElements::NavigateFocusRing {
    #[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
    pub type ChangeDirection = crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection;
    #[cfg(
        feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal"
    )]
    pub type FocusableHierarchyTraversal = crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal;
    pub fn GetFocusChangeDirection(
        &mut self,
        currentFocusable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusChangeDirection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        > = __cordl_object.invoke("GetFocusChangeDirection", (currentFocusable, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextFocusable(
        &mut self,
        currentFocusable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("GetNextFocusable", (currentFocusable, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextFocusable2D(
        &mut self,
        currentFocusable: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("GetNextFocusable2D", (currentFocusable, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsActive(
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsActive", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNavigable(
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNavigable", (focusable))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (root))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (root))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusController,
        > = __cordl_object.invoke("get_focusController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::NavigateFocusRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl AsRef<crate::UnityEngine::UIElements::IFocusRing>
for crate::UnityEngine::UIElements::NavigateFocusRing {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IFocusRing {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl AsMut<crate::UnityEngine::UIElements::IFocusRing>
for crate::UnityEngine::UIElements::NavigateFocusRing {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IFocusRing {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigateFocusRing_ChangeDirection {
    __cordl_parent: crate::UnityEngine::UIElements::FocusChangeDirection,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ChangeDirection";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection {
    type Target = crate::UnityEngine::UIElements::FocusChangeDirection;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
impl crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection {
    pub fn New(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (i))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (i))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavigateFocusRing_FocusableHierarchyTraversal {
    pub currentFocusable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub validRect: crate::UnityEngine::Rect,
    pub firstPass: bool,
    pub direction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "FocusableHierarchyTraversal";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
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
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal")]
impl crate::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal {
    pub fn GetBestOverall(
        &mut self,
        candidate: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        bestSoFar: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBestOverall",
            (candidate, bestSoFar),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Order(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Order",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StrictOrder_Rect_Rect1(
        &mut self,
        ra: crate::UnityEngine::Rect,
        rb: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StrictOrder",
            (ra, rb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StrictOrder_VisualElement_VisualElement0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StrictOrder",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TieBreaker(
        &mut self,
        ra: crate::UnityEngine::Rect,
        rb: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TieBreaker",
            (ra, rb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateElement(
        &mut self,
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateElement",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateHierarchyTraversal(
        &mut self,
        v: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateHierarchyTraversal",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
}

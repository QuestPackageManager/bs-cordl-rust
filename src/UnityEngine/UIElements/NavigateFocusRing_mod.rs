#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigateFocusRing_ChangeDirection {
    __cordl_parent: crate::UnityEngine::UIElements::FocusChangeDirection,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+ChangeDirection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection =>
    "UnityEngine.UIElements"."NavigateFocusRing/ChangeDirection"
);
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
    pub fn _ctor(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (i))?;
        Ok(__cordl_ret)
    }
    pub fn New(i: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (i))?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone)]
pub struct NavigateFocusRing_FocusableHierarchyTraversal {
    pub currentFocusable: *mut crate::UnityEngine::UIElements::VisualElement,
    pub validRect: crate::UnityEngine::Rect,
    pub firstPass: bool,
    pub direction: *mut crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing+FocusableHierarchyTraversal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::NavigateFocusRing_FocusableHierarchyTraversal =>
    "UnityEngine.UIElements"."NavigateFocusRing/FocusableHierarchyTraversal"
);
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
    pub fn ValidateHierarchyTraversal(
        &mut self,
        v: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateHierarchyTraversal",
            (v),
        )?;
        Ok(__cordl_ret)
    }
    pub fn StrictOrder_VisualElement_VisualElement0(
        &mut self,
        a: *mut crate::UnityEngine::UIElements::VisualElement,
        b: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "StrictOrder",
            (a, b),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetBestOverall(
        &mut self,
        candidate: *mut crate::UnityEngine::UIElements::VisualElement,
        bestSoFar: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBestOverall",
            (candidate, bestSoFar),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ValidateElement(
        &mut self,
        v: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ValidateElement",
            (v),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Order(
        &mut self,
        a: *mut crate::UnityEngine::UIElements::VisualElement,
        b: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Order",
            (a, b),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct NavigateFocusRing {
    __cordl_parent: crate::System::Object,
    pub m_Root: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_Ring: *mut crate::UnityEngine::UIElements::VisualElementFocusRing,
}
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::NavigateFocusRing =>
    "UnityEngine.UIElements"."NavigateFocusRing"
);
#[cfg(feature = "UnityEngine+UIElements+NavigateFocusRing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::NavigateFocusRing {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (root))?;
        Ok(__cordl_ret)
    }
    pub fn GetFocusChangeDirection(
        &mut self,
        currentFocusable: *mut crate::UnityEngine::UIElements::Focusable,
        e: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::FocusChangeDirection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::FocusChangeDirection = __cordl_object
            .invoke("GetFocusChangeDirection", (currentFocusable, e))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextFocusable2D(
        &mut self,
        currentFocusable: *mut crate::UnityEngine::UIElements::Focusable,
        direction: *mut crate::UnityEngine::UIElements::NavigateFocusRing_ChangeDirection,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Focusable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Focusable = __cordl_object
            .invoke("GetNextFocusable2D", (currentFocusable, direction))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextFocusable(
        &mut self,
        currentFocusable: *mut crate::UnityEngine::UIElements::Focusable,
        direction: *mut crate::UnityEngine::UIElements::FocusChangeDirection,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UIElements::Focusable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::Focusable = __cordl_object
            .invoke("GetNextFocusable", (currentFocusable, direction))?;
        Ok(__cordl_ret)
    }
    pub fn get_focusController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::FocusController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::FocusController = __cordl_object
            .invoke("get_focusController", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        root: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (root))?;
        Ok(__cordl_object)
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

#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementFocusRing {
    __cordl_parent: crate::System::Object,
    pub root: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _defaultFocusOrder_k__BackingField: crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder,
    pub m_FocusRing: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementFocusRing
    => "UnityEngine.UIElements"."VisualElementFocusRing"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementFocusRing {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementFocusRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
impl crate::UnityEngine::UIElements::VisualElementFocusRing {
    #[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+DefaultFocusOrder")]
    pub type DefaultFocusOrder = crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder;
    #[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
    pub type FocusRingRecord = crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord;
    pub fn BuildRingForScopeRecursive(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        scopeIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        scopeList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRingForScopeRecursive", (ve, scopeIndex, scopeList))?;
        Ok(__cordl_ret)
    }
    pub fn DoUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn FocusRingAutoIndexSort(
        &mut self,
        a: *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
        b: *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FocusRingAutoIndexSort", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn FocusRingSort(
        &mut self,
        a: *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
        b: *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FocusRingSort", (a, b))?;
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
    pub fn GetFocusableInternalIndex(
        &mut self,
        f: *mut crate::UnityEngine::UIElements::Focusable,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFocusableInternalIndex", (f))?;
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
    pub fn New(
        root: *mut crate::UnityEngine::UIElements::VisualElement,
        dfo: crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (root, dfo))?;
        Ok(__cordl_object)
    }
    pub fn SortAndFlattenScopeLists(
        &mut self,
        rootScopeList: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortAndFlattenScopeLists", (rootScopeList))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElement,
        dfo: crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (root, dfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_defaultFocusOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder = __cordl_object
            .invoke("get_defaultFocusOrder", ())?;
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
    pub fn set_defaultFocusOrder(
        &mut self,
        value: crate::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultFocusOrder", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementFocusRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+DefaultFocusOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualElementFocusRing_DefaultFocusOrder {
    ChildOrder = 0i32,
    PositionXY = 1i32,
    PositionYX = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+DefaultFocusOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementFocusRing_DefaultFocusOrder =>
    "UnityEngine.UIElements"."VisualElementFocusRing/DefaultFocusOrder"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementFocusRing_FocusRingRecord {
    __cordl_parent: crate::System::Object,
    pub m_AutoIndex: i32,
    pub m_Focusable: *mut crate::UnityEngine::UIElements::Focusable,
    pub m_IsSlot: bool,
    pub m_ScopeNavigationOrder: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord =>
    "UnityEngine.UIElements"."VisualElementFocusRing/FocusRingRecord"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
impl crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementFocusRing+FocusRingRecord")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementFocusRing_FocusRingRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

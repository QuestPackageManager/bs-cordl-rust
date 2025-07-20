#[cfg(feature = "SelectableCellSelectableStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectableCellSelectableStateController {
    __cordl_parent: crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
    >,
}
#[cfg(feature = "SelectableCellSelectableStateController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SelectableCellSelectableStateController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SelectableCellSelectableStateController";
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
#[cfg(feature = "SelectableCellSelectableStateController")]
impl std::ops::Deref
for crate::GlobalNamespace::SelectableCellSelectableStateController {
    type Target = crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectableCellSelectableStateController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SelectableCellSelectableStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectableCellSelectableStateController")]
impl crate::GlobalNamespace::SelectableCellSelectableStateController {
    pub fn HandleSelectableCellHighlightDidChange(
        &mut self,
        selectableCell: quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
                    crate::HMUI::SelectableCell_TransitionType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleSelectableCellHighlightDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSelectableCellHighlightDidChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (selectableCell, transitionType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectableCellInteractableDidChange(
        &mut self,
        interactableCell: quest_hook::libil2cpp::Gc<crate::HMUI::Interactable>,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::Interactable>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleSelectableCellInteractableDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSelectableCellInteractableDidChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (interactableCell, interactable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectableCellSelectionStateDidChange(
        &mut self,
        selectableCell: quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
        owner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
                    crate::HMUI::SelectableCell_TransitionType,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("HandleSelectableCellSelectionStateDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(),
                    "HandleSelectableCellSelectionStateDidChange", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (selectableCell, transitionType, owner))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(), "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(), "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveState(
        &mut self,
        selectableCell: quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
                    crate::HMUI::SelectableCell_TransitionType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ResolveState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(), "ResolveState", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (selectableCell, transitionType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::SelectableCellSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::SelectableCellSelectableStateController as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectableCellSelectableStateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectableCellSelectableStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

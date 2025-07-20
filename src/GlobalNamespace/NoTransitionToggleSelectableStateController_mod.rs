#[cfg(feature = "NoTransitionToggleSelectableStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct NoTransitionToggleSelectableStateController {
    __cordl_parent: crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::NoTransitionsToggle>,
    >,
}
#[cfg(feature = "NoTransitionToggleSelectableStateController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoTransitionToggleSelectableStateController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoTransitionToggleSelectableStateController";
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
#[cfg(feature = "NoTransitionToggleSelectableStateController")]
impl std::ops::Deref
for crate::GlobalNamespace::NoTransitionToggleSelectableStateController {
    type Target = crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::NoTransitionsToggle>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoTransitionToggleSelectableStateController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoTransitionToggleSelectableStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoTransitionToggleSelectableStateController")]
impl crate::GlobalNamespace::NoTransitionToggleSelectableStateController {
    pub fn HandleNoTransitionToggleSelectionStateDidChange(
        &mut self,
        state: crate::HMUI::UISelectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoTransitionToggleSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::HMUI::UISelectionState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleNoTransitionToggleSelectionStateDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoTransitionToggleSelectableStateController
                    as quest_hook::libil2cpp::Type > ::class(),
                    "HandleNoTransitionToggleSelectionStateDidChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state))?
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoTransitionToggleSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoTransitionToggleSelectableStateController
                    as quest_hook::libil2cpp::Type > ::class(), "OnDisable", 0usize
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoTransitionToggleSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoTransitionToggleSelectableStateController
                    as quest_hook::libil2cpp::Type > ::class(), "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSelectionState(
        &mut self,
        state: crate::HMUI::UISelectionState,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoTransitionToggleSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::HMUI::UISelectionState, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ResolveSelectionState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoTransitionToggleSelectableStateController
                    as quest_hook::libil2cpp::Type > ::class(), "ResolveSelectionState",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, animated))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoTransitionToggleSelectableStateController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoTransitionToggleSelectableStateController
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoTransitionToggleSelectableStateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoTransitionToggleSelectableStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

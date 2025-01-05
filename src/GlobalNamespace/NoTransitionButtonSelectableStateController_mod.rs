#[cfg(feature = "NoTransitionButtonSelectableStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct NoTransitionButtonSelectableStateController {
    __cordl_parent: crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::NoTransitionsButton>,
    >,
}
#[cfg(feature = "NoTransitionButtonSelectableStateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoTransitionButtonSelectableStateController => ""
    ."NoTransitionButtonSelectableStateController"
);
#[cfg(feature = "NoTransitionButtonSelectableStateController")]
impl std::ops::Deref
for crate::GlobalNamespace::NoTransitionButtonSelectableStateController {
    type Target = crate::GlobalNamespace::SelectableStateController_1<
        quest_hook::libil2cpp::Gc<crate::HMUI::NoTransitionsButton>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoTransitionButtonSelectableStateController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoTransitionButtonSelectableStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoTransitionButtonSelectableStateController")]
impl crate::GlobalNamespace::NoTransitionButtonSelectableStateController {
    pub fn HandleNoTransitionButtonSelectionStateDidChange(
        &mut self,
        state: crate::HMUI::NoTransitionsButton_SelectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoTransitionButtonSelectionStateDidChange", (state))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSelectionState(
        &mut self,
        state: crate::HMUI::NoTransitionsButton_SelectionState,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveSelectionState", (state, animated))?;
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
#[cfg(feature = "NoTransitionButtonSelectableStateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoTransitionButtonSelectableStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

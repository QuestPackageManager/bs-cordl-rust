#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
#[repr(C)]
#[derive(Debug)]
pub struct IModifiableState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::UI::IModifiableState =>
    "Tayx.Graphy.UI"."IModifiableState"
);
#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
impl std::ops::Deref for crate::Tayx::Graphy::UI::IModifiableState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
impl std::ops::DerefMut for crate::Tayx::Graphy::UI::IModifiableState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
impl crate::Tayx::Graphy::UI::IModifiableState {
    pub fn SetState(
        &mut self,
        newState: crate::Tayx::Graphy::GraphyManager_ModuleState,
        silentUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetState", (newState, silentUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Tayx+Graphy+UI+IModifiableState")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::UI::IModifiableState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

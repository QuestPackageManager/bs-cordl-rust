#[cfg(feature = "SelectSubMenuDestination+Destination")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectSubMenuDestination_Destination {
    Campaign = 1i32,
    MainMenu = 0i32,
    Multiplayer = 6i32,
    PartyFreePlay = 3i32,
    Settings = 4i32,
    SoloFreePlay = 2i32,
    Tutorial = 5i32,
}
#[cfg(feature = "SelectSubMenuDestination+Destination")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectSubMenuDestination_Destination => ""
    ."SelectSubMenuDestination/Destination"
);
#[cfg(feature = "SelectSubMenuDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectSubMenuDestination {
    __cordl_parent: MenuDestination,
    pub menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
}
#[cfg(feature = "SelectSubMenuDestination")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SelectSubMenuDestination => ""
    ."SelectSubMenuDestination"
);
#[cfg(feature = "SelectSubMenuDestination")]
impl std::ops::Deref for SelectSubMenuDestination {
    type Target = MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectSubMenuDestination")]
impl std::ops::DerefMut for SelectSubMenuDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectSubMenuDestination")]
impl SelectSubMenuDestination {
    #[cfg(feature = "SelectSubMenuDestination+Destination")]
    pub type Destination = crate::GlobalNamespace::SelectSubMenuDestination_Destination;
    pub fn _ctor(
        &mut self,
        menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (menuDestination))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        menuDestination: crate::GlobalNamespace::SelectSubMenuDestination_Destination,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (menuDestination))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SelectSubMenuDestination")]
impl quest_hook::libil2cpp::ObjectType for SelectSubMenuDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

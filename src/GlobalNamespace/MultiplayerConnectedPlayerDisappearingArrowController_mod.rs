#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerDisappearingArrowController {
    __cordl_parent: crate::GlobalNamespace::DisappearingArrowControllerBase_1<
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _gameNoteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerDisappearingArrowController => ""
    ."MultiplayerConnectedPlayerDisappearingArrowController"
);
#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerDisappearingArrowController {
    type Target = crate::GlobalNamespace::DisappearingArrowControllerBase_1<
        *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerDisappearingArrowController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerDisappearingArrowController {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_gameNoteController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
        > = __cordl_object.invoke("get_gameNoteController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerDisappearingArrowController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerDisappearingArrowController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

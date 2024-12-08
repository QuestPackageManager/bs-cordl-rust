#[cfg(feature = "MultiplayerLayoutProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLayoutProvider {
    __cordl_parent: crate::System::Object,
    pub _layout_k__BackingField: crate::GlobalNamespace::MultiplayerPlayerLayout,
    pub _activePlayerSpotsCount_k__BackingField: i32,
    pub playersLayoutWasCalculatedEvent: *mut crate::System::Action_2<
        crate::GlobalNamespace::MultiplayerPlayerLayout,
        i32,
    >,
}
#[cfg(feature = "MultiplayerLayoutProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLayoutProvider => ""
    ."MultiplayerLayoutProvider"
);
#[cfg(feature = "MultiplayerLayoutProvider")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLayoutProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLayoutProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLayoutProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLayoutProvider")]
impl crate::GlobalNamespace::MultiplayerLayoutProvider {
    pub fn CalculateLayout(
        &mut self,
        activePlayersCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerPlayerLayout> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerPlayerLayout = __cordl_object
            .invoke("CalculateLayout", (activePlayersCount))?;
        Ok(__cordl_ret)
    }
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
    pub fn add_playersLayoutWasCalculatedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::GlobalNamespace::MultiplayerPlayerLayout,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playersLayoutWasCalculatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activePlayerSpotsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_activePlayerSpotsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerPlayerLayout> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerPlayerLayout = __cordl_object
            .invoke("get_layout", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_playersLayoutWasCalculatedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            crate::GlobalNamespace::MultiplayerPlayerLayout,
            i32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playersLayoutWasCalculatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_activePlayerSpotsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activePlayerSpotsCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_layout(
        &mut self,
        value: crate::GlobalNamespace::MultiplayerPlayerLayout,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layout", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLayoutProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLayoutProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

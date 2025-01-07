#[cfg(feature = "MultiplayerMockSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerMockSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _localPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockPlayerSettings,
    >,
    pub _otherPlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlayerSettings>,
        >,
    >,
    pub _quickplayServer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MockServerSettings,
    >,
    pub _multiplayerStatusData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerStatusData,
    >,
    pub _quickPlaySetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::QuickPlaySetupData,
    >,
}
#[cfg(feature = "MultiplayerMockSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerMockSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerMockSettings";
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
#[cfg(feature = "MultiplayerMockSettings")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerMockSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerMockSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl crate::GlobalNamespace::MultiplayerMockSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SharedSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerMockSettings>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerMockSettings,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("SharedSettings", ())?;
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
    pub fn get_localPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlayerSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockPlayerSettings,
        > = __cordl_object.invoke("get_localPlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerStatusData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerStatusData,
        > = __cordl_object.invoke("get_multiplayerStatusData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlayerSettings>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockPlayerSettings>,
            >,
        > = __cordl_object.invoke("get_otherPlayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_quickPlaySetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::QuickPlaySetupData,
        > = __cordl_object.invoke("get_quickPlaySetupData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_quickplayServer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MockServerSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MockServerSettings,
        > = __cordl_object.invoke("get_quickplayServer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_multiplayerStatusData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerStatusData", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_quickPlaySetupData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_quickPlaySetupData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerMockSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

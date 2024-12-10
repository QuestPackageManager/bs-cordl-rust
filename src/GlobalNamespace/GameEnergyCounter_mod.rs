#[cfg(feature = "GameEnergyCounter")]
#[repr(C)]
#[derive(Debug)]
pub struct GameEnergyCounter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _batteryLives: i32,
    pub _initData: *mut crate::GlobalNamespace::GameEnergyCounter_InitData,
    pub _saberClashChecker: *mut crate::GlobalNamespace::SaberClashChecker,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _playerHeadAndObstacleInteraction: *mut crate::GlobalNamespace::PlayerHeadAndObstacleInteraction,
    pub didInitEvent: *mut crate::System::Action,
    pub gameEnergyDidReach0Event: *mut crate::System::Action,
    pub gameEnergyDidChangeEvent: *mut crate::System::Action_1<f32>,
    pub _energy_k__BackingField: f32,
    pub _energyType_k__BackingField: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    pub _noFail_k__BackingField: bool,
    pub _instaFail_k__BackingField: bool,
    pub _failOnSaberClash_k__BackingField: bool,
    pub _isInitialized: bool,
    pub _didReach0Energy: bool,
    pub _nextFrameEnergyChange: f32,
}
#[cfg(feature = "GameEnergyCounter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameEnergyCounter => ""
    ."GameEnergyCounter"
);
#[cfg(feature = "GameEnergyCounter")]
impl std::ops::Deref for crate::GlobalNamespace::GameEnergyCounter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameEnergyCounter")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameEnergyCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameEnergyCounter")]
impl crate::GlobalNamespace::GameEnergyCounter {
    pub const kBadBurstSliderElementEnergyDrain: f32 = 0.025f32;
    pub const kBadNoteEnergyDrain: f32 = 0.1f32;
    pub const kGoodBurstSliderElementCharge: f32 = 0.002f32;
    pub const kGoodNoteEnergyCharge: f32 = 0.01f32;
    pub const kHitBombEnergyDrain: f32 = 0.15f32;
    pub const kMissBurstSliderElementEnergyDrain: f32 = 0.03f32;
    pub const kMissNoteEnergyDrain: f32 = 0.15f32;
    pub const kObstacleEnergyDrainPerSecond: f32 = 1.3f32;
    #[cfg(feature = "GameEnergyCounter+InitData")]
    pub type InitData = crate::GlobalNamespace::GameEnergyCounter_InitData;
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteController,
        >,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessEnergyChange(
        &mut self,
        energyChange: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEnergyChange", (energyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn add_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInitEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_gameEnergyDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameEnergyDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_gameEnergyDidReach0Event(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameEnergyDidReach0Event", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryEnergy(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_batteryEnergy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_batteryLives(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_batteryLives", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_energy(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_energy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_energyType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayModifiers_EnergyType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayModifiers_EnergyType = __cordl_object
            .invoke("get_energyType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_failOnSaberClash(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_failOnSaberClash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instaFail(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_instaFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noFail(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInitEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameEnergyDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameEnergyDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_gameEnergyDidReach0Event(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameEnergyDidReach0Event", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_energy(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_energy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_energyType(
        &mut self,
        value: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_energyType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_failOnSaberClash(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_failOnSaberClash", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_instaFail(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_instaFail", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_noFail(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_noFail", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameEnergyCounter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameEnergyCounter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameEnergyCounter")]
impl AsRef<crate::GlobalNamespace::IGameEnergyCounter>
for crate::GlobalNamespace::GameEnergyCounter {
    fn as_ref(&self) -> &crate::GlobalNamespace::IGameEnergyCounter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameEnergyCounter")]
impl AsMut<crate::GlobalNamespace::IGameEnergyCounter>
for crate::GlobalNamespace::GameEnergyCounter {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IGameEnergyCounter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameEnergyCounter+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameEnergyCounter_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    pub noFail: bool,
    pub instaFail: bool,
    pub failOnSaberClash: bool,
}
#[cfg(feature = "GameEnergyCounter+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameEnergyCounter_InitData =>
    ""."GameEnergyCounter/InitData"
);
#[cfg(feature = "GameEnergyCounter+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::GameEnergyCounter_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameEnergyCounter+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameEnergyCounter_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameEnergyCounter+InitData")]
impl crate::GlobalNamespace::GameEnergyCounter_InitData {
    pub fn New(
        energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
        noFail: bool,
        instaFail: bool,
        failOnSaberClash: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (energyType, noFail, instaFail, failOnSaberClash))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
        noFail: bool,
        instaFail: bool,
        failOnSaberClash: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (energyType, noFail, instaFail, failOnSaberClash))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameEnergyCounter+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameEnergyCounter_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

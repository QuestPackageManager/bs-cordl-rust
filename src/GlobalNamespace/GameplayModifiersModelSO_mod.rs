#[cfg(feature = "GameplayModifiersModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _batteryEnergy: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _instaFail: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _noObstacles: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _noBombs: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _fastNotes: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _strictAngles: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _disappearingArrows: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _fasterSong: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _slowerSong: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _noArrows: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _ghostNotes: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _noFailOn0Energy: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _superFastSong: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _proMode: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _zenMode: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _smallCubes: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifierParamsSO,
    >,
    pub _gameplayModifierGetters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            *mut crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
        >,
    >,
}
#[cfg(feature = "GameplayModifiersModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayModifiersModelSO => ""
    ."GameplayModifiersModelSO"
);
#[cfg(feature = "GameplayModifiersModelSO")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifiersModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifiersModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO")]
impl crate::GlobalNamespace::GameplayModifiersModelSO {
    pub const kMaxPossibleMultiplier: f32 = 1.21f32;
    #[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
    pub type GameplayModifierBoolGetter = crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter;
    pub fn CreateGameplayModifiers(
        &mut self,
        valueGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object.invoke("CreateGameplayModifiers", (valueGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateModifierParamsList(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        > = __cordl_object.invoke("CreateModifierParamsList", (gameplayModifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameplayModifierParams(
        &mut self,
        modifier: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifierParamsSO,
        > = __cordl_object.invoke("GetGameplayModifierParams", (modifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedScoreForGameplayModifiers(
        &mut self,
        multipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "GetModifiedScoreForGameplayModifiers",
                (multipliedScore, modifierParams, energy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetModifierBoolValue(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        gameplayModifierParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifierParamsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "GetModifierBoolValue",
                (gameplayModifiers, gameplayModifierParams),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSongSpeedFromValueGetter(
        &mut self,
        valueGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayModifiers_SongSpeed = __cordl_object
            .invoke("GetSongSpeedFromValueGetter", (valueGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalMultiplier(
        &mut self,
        modifierParams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetTotalMultiplier", (modifierParams, energy))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaxModifiedScoreForMaxMultipliedScore_GameplayModifiersModelSO_f32_1(
        &mut self,
        maxMultipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        >,
        gameplayModifiersModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiersModelSO,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "MaxModifiedScoreForMaxMultipliedScore",
                (maxMultipliedScore, modifierParams, gameplayModifiersModel, energy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MaxModifiedScoreForMaxMultipliedScore_f32_0(
        &mut self,
        maxMultipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
            >,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "MaxModifiedScoreForMaxMultipliedScore",
                (maxMultipliedScore, modifierParams, energy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_gameplayModifierGetters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
                *mut crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::GlobalNamespace::GameplayModifierParamsSO,
                *mut crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
            >,
        > = __cordl_object.invoke("get_gameplayModifierGetters", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifiersModelSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersModelSO_GameplayModifierBoolGetter {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter => ""
    ."GameplayModifiersModelSO/GameplayModifierBoolGetter"
);
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    pub fn BeginInvoke(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (gameplayModifiers, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Invoke", (gameplayModifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

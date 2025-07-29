#[cfg(feature = "cordl_class_GameplayModifiersModelSO")]
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
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
        >,
    >,
}
#[cfg(feature = "cordl_class_GameplayModifiersModelSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiersModelSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiersModelSO";
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
#[cfg(feature = "GameplayModifiersModelSO")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifiersModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifiersModelSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifierParamsSO,
                            >,
                            bool,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayModifiers,
                        >,
                        1usize,
                    >("CreateGameplayModifiers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateGameplayModifiers", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (valueGetter))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateModifierParamsList(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayModifiers,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifierParamsSO,
                            >,
                        >,
                        1usize,
                    >("CreateModifierParamsList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateModifierParamsList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (gameplayModifiers))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGameplayModifierParams(
        &mut self,
        modifier: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::GameplayModifierMask),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayModifierParamsSO,
                        >,
                        1usize,
                    >("GetGameplayModifierParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGameplayModifierParams", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifierParamsSO,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (modifier))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetModifiedScoreForGameplayModifiers(
        &mut self,
        multipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::GameplayModifierParamsSO,
                                >,
                            >,
                            f32,
                        ),
                        i32,
                        3usize,
                    >("GetModifiedScoreForGameplayModifiers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetModifiedScoreForGameplayModifiers", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (multipliedScore, modifierParams, energy))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifierParamsSO,
                            >,
                        ),
                        bool,
                        2usize,
                    >("GetModifierBoolValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetModifierBoolValue", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (gameplayModifiers, gameplayModifierParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSongSpeedFromValueGetter(
        &mut self,
        valueGetter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifierParamsSO,
                            >,
                            bool,
                        >),
                        crate::GlobalNamespace::GameplayModifiers_SongSpeed,
                        1usize,
                    >("GetSongSpeedFromValueGetter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSongSpeedFromValueGetter", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::GameplayModifiers_SongSpeed = unsafe {
            cordl_method_info.invoke_unchecked(self, (valueGetter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalMultiplier(
        &mut self,
        modifierParams: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::GameplayModifierParamsSO,
                                >,
                            >,
                            f32,
                        ),
                        f32,
                        2usize,
                    >("GetTotalMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTotalMultiplier", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (modifierParams, energy))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MaxModifiedScoreForMaxMultipliedScore_GameplayModifiersModelSO_f32_1(
        &mut self,
        maxMultipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        >,
        gameplayModifiersModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiersModelSO,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::GameplayModifierParamsSO,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiersModelSO,
                            >,
                            f32,
                        ),
                        i32,
                        4usize,
                    >("MaxModifiedScoreForMaxMultipliedScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MaxModifiedScoreForMaxMultipliedScore", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (maxMultipliedScore, modifierParams, gameplayModifiersModel, energy),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MaxModifiedScoreForMaxMultipliedScore_f32_0(
        &mut self,
        maxMultipliedScore: i32,
        modifierParams: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
        >,
        energy: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::GameplayModifierParamsSO,
                                >,
                            >,
                            f32,
                        ),
                        i32,
                        3usize,
                    >("MaxModifiedScoreForMaxMultipliedScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MaxModifiedScoreForMaxMultipliedScore", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (maxMultipliedScore, modifierParams, energy))?
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEnable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifierGetters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifierParamsSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
                            >,
                        >,
                        0usize,
                    >("get_gameplayModifierGetters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_gameplayModifierGetters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifierParamsSO>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_GameplayModifiersModelSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_GameplayModifiersModelSO+GameplayModifierBoolGetter")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiersModelSO_GameplayModifierBoolGetter {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "cordl_class_GameplayModifiersModelSO+GameplayModifierBoolGetter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiersModelSO/GameplayModifierBoolGetter";
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
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::GameplayModifiers,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (gameplayModifiers, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        bool,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayModifiers,
                        >),
                        bool,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (gameplayModifiers))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_GameplayModifiersModelSO+GameplayModifierBoolGetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayModifiersModelSO_GameplayModifierBoolGetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

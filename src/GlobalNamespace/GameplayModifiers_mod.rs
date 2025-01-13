#[cfg(feature = "GameplayModifiers")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayModifiers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
    pub _noFailOn0Energy: bool,
    pub _instaFail: bool,
    pub _failOnSaberClash: bool,
    pub _enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
    pub _fastNotes: bool,
    pub _strictAngles: bool,
    pub _disappearingArrows: bool,
    pub _ghostNotes: bool,
    pub _noBombs: bool,
    pub _songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    pub _noArrows: bool,
    pub _proMode: bool,
    pub _zenMode: bool,
    pub _smallCubes: bool,
}
#[cfg(feature = "GameplayModifiers")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::GameplayModifiers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiers";
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
#[cfg(feature = "GameplayModifiers")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayModifiers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiers")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayModifiers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayModifiers")]
impl crate::GlobalNamespace::GameplayModifiers {
    #[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
    pub type EnabledObstacleType = crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType;
    #[cfg(feature = "GameplayModifiers+EnergyType")]
    pub type EnergyType = crate::GlobalNamespace::GameplayModifiers_EnergyType;
    #[cfg(feature = "GameplayModifiers+SongSpeed")]
    pub type SongSpeed = crate::GlobalNamespace::GameplayModifiers_SongSpeed;
    pub fn AreValuesEqual(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreValuesEqual", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyWith(
        &mut self,
        energyType: crate::System::Nullable_1<
            crate::GlobalNamespace::GameplayModifiers_EnergyType,
        >,
        noFailOn0Energy: crate::System::Nullable_1<bool>,
        instaFail: crate::System::Nullable_1<bool>,
        failOnSaberClash: crate::System::Nullable_1<bool>,
        enabledObstacleType: crate::System::Nullable_1<
            crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
        >,
        noBombs: crate::System::Nullable_1<bool>,
        fastNotes: crate::System::Nullable_1<bool>,
        strictAngles: crate::System::Nullable_1<bool>,
        disappearingArrows: crate::System::Nullable_1<bool>,
        songSpeed: crate::System::Nullable_1<
            crate::GlobalNamespace::GameplayModifiers_SongSpeed,
        >,
        noArrows: crate::System::Nullable_1<bool>,
        ghostNotes: crate::System::Nullable_1<bool>,
        proMode: crate::System::Nullable_1<bool>,
        zenMode: crate::System::Nullable_1<bool>,
        smallCubes: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object
            .invoke(
                "CopyWith",
                (
                    energyType,
                    noFailOn0Energy,
                    instaFail,
                    failOnSaberClash,
                    enabledObstacleType,
                    noBombs,
                    fastNotes,
                    strictAngles,
                    disappearingArrows,
                    songSpeed,
                    noArrows,
                    ghostNotes,
                    proMode,
                    zenMode,
                    smallCubes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromSerializedData(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromSerializedData", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromGameplayModifierMask(
        mask: crate::GlobalNamespace::GameplayModifierMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromGameplayModifierMask", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWithoutModifiers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWithoutModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LiteNetLib_Utils_INetImmutableSerializable_GameplayModifiers__CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object
            .invoke(
                "LiteNetLib.Utils.INetImmutableSerializable<GameplayModifiers>.CreateFromSerializedData",
                (reader),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_GameplayModifiers_EnergyType__cordl_bool__cordl_bool__cordl_bool_GameplayModifiers_EnabledObstacleType__cordl_bool__cordl_bool__cordl_bool__cordl_bool_GameplayModifiers_SongSpeed__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
        noFailOn0Energy: bool,
        instaFail: bool,
        failOnSaberClash: bool,
        enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
        noBombs: bool,
        fastNotes: bool,
        strictAngles: bool,
        disappearingArrows: bool,
        songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
        noArrows: bool,
        ghostNotes: bool,
        proMode: bool,
        zenMode: bool,
        smallCubes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    energyType,
                    noFailOn0Energy,
                    instaFail,
                    failOnSaberClash,
                    enabledObstacleType,
                    noBombs,
                    fastNotes,
                    strictAngles,
                    disappearingArrows,
                    songSpeed,
                    noArrows,
                    ghostNotes,
                    proMode,
                    zenMode,
                    smallCubes,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GameplayModifiers_EnergyType__cordl_bool__cordl_bool__cordl_bool_GameplayModifiers_EnabledObstacleType__cordl_bool__cordl_bool__cordl_bool__cordl_bool_GameplayModifiers_SongSpeed__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        energyType: crate::GlobalNamespace::GameplayModifiers_EnergyType,
        noFailOn0Energy: bool,
        instaFail: bool,
        failOnSaberClash: bool,
        enabledObstacleType: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
        noBombs: bool,
        fastNotes: bool,
        strictAngles: bool,
        disappearingArrows: bool,
        songSpeed: crate::GlobalNamespace::GameplayModifiers_SongSpeed,
        noArrows: bool,
        ghostNotes: bool,
        proMode: bool,
        zenMode: bool,
        smallCubes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    energyType,
                    noFailOn0Energy,
                    instaFail,
                    failOnSaberClash,
                    enabledObstacleType,
                    noBombs,
                    fastNotes,
                    strictAngles,
                    disappearingArrows,
                    songSpeed,
                    noArrows,
                    ghostNotes,
                    proMode,
                    zenMode,
                    smallCubes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cutAngleTolerance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cutAngleTolerance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disappearingArrows(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disappearingArrows", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabledObstacleType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType = __cordl_object
            .invoke("get_enabledObstacleType", ())?;
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
    pub fn get_fastNotes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fastNotes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ghostNotes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ghostNotes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instaFail(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_instaFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noArrows(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noArrows", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noBombs(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noBombs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noFailOn0Energy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_noFailOn0Energy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_notesUniformScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_notesUniformScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_proMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_proMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_smallCubes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_smallCubes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songSpeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameplayModifiers_SongSpeed,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameplayModifiers_SongSpeed = __cordl_object
            .invoke("get_songSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songSpeedMul(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songSpeedMul", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_strictAngles(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_strictAngles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zenMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_zenMode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayModifiers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameplayModifiers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayModifiers")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    >,
> for crate::GlobalNamespace::GameplayModifiers {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayModifiers")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    >,
> for crate::GlobalNamespace::GameplayModifiers {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_EnabledObstacleType {
    #[default]
    All = 0i32,
    FullHeightOnly = 1i32,
    NoObstacles = 2i32,
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiers/EnabledObstacleType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "GameplayModifiers+EnabledObstacleType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_EnabledObstacleType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_EnergyType {
    #[default]
    Bar = 0i32,
    Battery = 1i32,
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_EnergyType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiers/EnergyType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_EnergyType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_EnergyType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_EnergyType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "GameplayModifiers+EnergyType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_EnergyType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayModifiers_SongSpeed {
    #[default]
    Faster = 1i32,
    Normal = 0i32,
    Slower = 2i32,
    SuperFast = 3i32,
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameplayModifiers_SongSpeed {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameplayModifiers/SongSpeed";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameplayModifiers_SongSpeed {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameplayModifiers_SongSpeed {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameplayModifiers_SongSpeed {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "GameplayModifiers+SongSpeed")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameplayModifiers_SongSpeed {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

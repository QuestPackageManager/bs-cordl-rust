#[cfg(feature = "MultiplayerLevelCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelCompletionResults {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
    pub _playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
    pub _levelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelCompletionResults,
    >,
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelCompletionResults";
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
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    #[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
    pub type MultiplayerPlayerLevelEndReason = crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason;
    #[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
    pub type MultiplayerPlayerLevelEndState = crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState;
    pub fn CompareTo(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        > = __cordl_object.invoke("CreateFromSerializedData", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAnyResult(
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAnyResult", (playerLevelEndState))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason_LevelCompletionResults1(
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
        playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (playerLevelEndState, playerLevelEndReason, levelCompletionResults),
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
    pub fn _ctor_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason_LevelCompletionResults1(
        &mut self,
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
        playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (playerLevelEndState, playerLevelEndReason, levelCompletionResults),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_failedOrGivenUp(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_failedOrGivenUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasAnyResults(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasAnyResults", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelCompletionResults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = __cordl_object.invoke("get_levelCompletionResults", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerLevelEndReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason = __cordl_object
            .invoke("get_playerLevelEndReason", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerLevelEndState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState = __cordl_object
            .invoke("get_playerLevelEndState", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl AsRef<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn as_ref(
        &self,
    ) -> &crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl AsMut<
    crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn as_mut(
        &mut self,
    ) -> &mut crate::LiteNetLib::Utils::INetImmutableSerializable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl AsRef<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl AsMut<crate::System::IComparable>
for crate::GlobalNamespace::MultiplayerLevelCompletionResults {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
    #[default]
    Cleared = 0i32,
    ConnectedAfterLevelEnded = 7i32,
    Failed = 1i32,
    GivenUp = 2i32,
    HostEndedLevel = 4i32,
    Quit = 3i32,
    StartupFailed = 6i32,
    WasInactive = 5i32,
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelCompletionResults/MultiplayerPlayerLevelEndReason";
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
    #[default]
    NotFinished = 1i32,
    NotStarted = 2i32,
    SongFinished = 0i32,
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelCompletionResults/MultiplayerPlayerLevelEndState";
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
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
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
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

#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapObjectAvoidanceYOffsetEvaluator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _currentYJumpOffsetBufferEndIndex: i32,
    pub _jumpDurationToDesiredZPosition: f32,
    pub _yJumpOffsetBuffer: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData,
        >,
    >,
    pub _audioTimeSource: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAudioTimeSource,
    >,
    pub _jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IJumpOffsetYProvider,
    >,
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapObjectAvoidanceYOffsetEvaluator";
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
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
impl crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator {
    pub const kYJumpOffsetBufferLength: i32 = 2000i32;
    pub const kYJumpOffsetBufferSongTimeInitValue: f32 = -1000f32;
    #[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
    pub type BufferData = crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData;
    pub fn GetJumpOffsetYAtJumpStartSongTime(
        &mut self,
        lastDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetJumpOffsetYAtJumpStartSongTime", (lastDeltaTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IJumpOffsetYProvider,
        >,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVariableMovementDataProvider,
        >,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    audioTimeSource,
                    jumpOffsetYProvider,
                    variableMovementDataProvider,
                    moveToPlayerHeadTParam,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioTimeSource: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IAudioTimeSource,
        >,
        jumpOffsetYProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IJumpOffsetYProvider,
        >,
        variableMovementDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVariableMovementDataProvider,
        >,
        moveToPlayerHeadTParam: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    audioTimeSource,
                    jumpOffsetYProvider,
                    variableMovementDataProvider,
                    moveToPlayerHeadTParam,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
    pub songTime: f32,
    pub yOffset: f32,
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BufferData";
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
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
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
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
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
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
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
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapObjectAvoidanceYOffsetEvaluator+BufferData")]
impl crate::GlobalNamespace::BeatmapObjectAvoidanceYOffsetEvaluator_BufferData {
    pub fn _ctor(
        &mut self,
        songTime: f32,
        yOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (songTime, yOffset),
        )?;
        Ok(__cordl_ret.into())
    }
}

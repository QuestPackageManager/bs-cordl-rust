#[cfg(feature = "SliderController+LengthType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderController_LengthType {
    Long = 2i32,
    Medium = 1i32,
    Short = 0i32,
}
#[cfg(feature = "SliderController+LengthType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderController_LengthType =>
    ""."SliderController/LengthType"
);
#[cfg(feature = "SliderController+Pool+Long")]
#[repr(C)]
#[derive(Debug)]
pub struct Pool_Long {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >,
}
#[cfg(feature = "SliderController+Pool+Long")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Pool_Long => ""
    ."SliderController/Pool/Long"
);
#[cfg(feature = "SliderController+Pool+Long")]
impl std::ops::Deref for crate::GlobalNamespace::Pool_Long {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Long")]
impl std::ops::DerefMut for crate::GlobalNamespace::Pool_Long {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Long")]
impl crate::GlobalNamespace::Pool_Long {
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
}
#[cfg(feature = "SliderController+Pool+Long")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Pool_Long {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderController+Pool+Medium")]
#[repr(C)]
#[derive(Debug)]
pub struct Pool_Medium {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >,
}
#[cfg(feature = "SliderController+Pool+Medium")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Pool_Medium => ""
    ."SliderController/Pool/Medium"
);
#[cfg(feature = "SliderController+Pool+Medium")]
impl std::ops::Deref for crate::GlobalNamespace::Pool_Medium {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Medium")]
impl std::ops::DerefMut for crate::GlobalNamespace::Pool_Medium {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Medium")]
impl crate::GlobalNamespace::Pool_Medium {
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
}
#[cfg(feature = "SliderController+Pool+Medium")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Pool_Medium {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderController_Pool {
    __cordl_parent: crate::System::Object,
    pub _shortPool: *mut crate::GlobalNamespace::Pool_Short,
    pub _mediumPool: *mut crate::GlobalNamespace::Pool_Medium,
    pub _longPool: *mut crate::GlobalNamespace::Pool_Long,
}
#[cfg(feature = "SliderController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderController_Pool => ""
    ."SliderController/Pool"
);
#[cfg(feature = "SliderController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::SliderController_Pool {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool")]
impl crate::GlobalNamespace::SliderController_Pool {
    pub const kMinDistanceToUseLong: f32 = 15f32;
    pub const kMinDistanceToUseMedium: f32 = 5f32;
    #[cfg(feature = "SliderController+Pool+Long")]
    pub type Long = crate::GlobalNamespace::Pool_Long;
    #[cfg(feature = "SliderController+Pool+Medium")]
    pub type Medium = crate::GlobalNamespace::Pool_Medium;
    #[cfg(feature = "SliderController+Pool+Short")]
    pub type Short = crate::GlobalNamespace::Pool_Short;
    pub fn GetPool(
        &mut self,
        lengthType: crate::GlobalNamespace::SliderController_LengthType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::MonoMemoryPool_1<
            *mut crate::GlobalNamespace::SliderController,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::MonoMemoryPool_1<
            *mut crate::GlobalNamespace::SliderController,
        > = __cordl_object.invoke("GetPool", (lengthType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        shortPool: *mut crate::GlobalNamespace::Pool_Short,
        mediumPool: *mut crate::GlobalNamespace::Pool_Medium,
        longPool: *mut crate::GlobalNamespace::Pool_Long,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (shortPool, mediumPool, longPool))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        shortPool: *mut crate::GlobalNamespace::Pool_Short,
        mediumPool: *mut crate::GlobalNamespace::Pool_Medium,
        longPool: *mut crate::GlobalNamespace::Pool_Long,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (shortPool, mediumPool, longPool))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SliderController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderController+Pool+Short")]
#[repr(C)]
#[derive(Debug)]
pub struct Pool_Short {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >,
}
#[cfg(feature = "SliderController+Pool+Short")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Pool_Short => ""
    ."SliderController/Pool/Short"
);
#[cfg(feature = "SliderController+Pool+Short")]
impl std::ops::Deref for crate::GlobalNamespace::Pool_Short {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::SliderController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Short")]
impl std::ops::DerefMut for crate::GlobalNamespace::Pool_Short {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController+Pool+Short")]
impl crate::GlobalNamespace::Pool_Short {
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
}
#[cfg(feature = "SliderController+Pool+Short")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Pool_Short {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderController")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderController {
    __cordl_parent: crate::GlobalNamespace::SliderControllerBase,
    pub _sliderIntensityEffect: *mut crate::GlobalNamespace::SliderIntensityEffect,
    pub _sliderMeshController: *mut crate::GlobalNamespace::SliderMeshController,
    pub _sliderMovement: *mut crate::GlobalNamespace::SliderMovement,
    pub _closeInteractionSaberPosSmoothParam: f32,
    pub _beatmapObjectSpawnController: *mut crate::GlobalNamespace::IBeatmapObjectSpawnController,
    pub _colorManager: *mut crate::GlobalNamespace::ColorManager,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
    pub _saberManager: *mut crate::GlobalNamespace::SaberManager,
    pub _sliderDidFinishMovement: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::ISliderDidFinishJumpEvent,
    >,
    pub _sliderDidStartDissolving: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::ISliderDidStartDissolvingEvent,
    >,
    pub _sliderDidDissolve: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::ISliderDidDissolveEvent,
    >,
    pub _sliderHeadDidMovePastCutMark: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::ISliderHeadDidMovePastCutMarkEvent,
    >,
    pub _sliderTailDidMovePastCutMark: *mut crate::GlobalNamespace::LazyCopyHashSet_1<
        *mut crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent,
    >,
    pub _lengthType: crate::GlobalNamespace::SliderController_LengthType,
    pub _sliderData: *mut crate::GlobalNamespace::SliderData,
    pub _saber: *mut crate::GlobalNamespace::Saber,
    pub _headJumpOffsetY: f32,
    pub _sliderDuration: f32,
    pub _initColor: crate::UnityEngine::Color,
    pub _attractingSaber: bool,
    pub _randomValue: f32,
    pub _zDistanceBetweenNotes: f32,
    pub _jumpDistance: f32,
    pub _closeSmoothedSaberInteractionPos: *mut crate::GlobalNamespace::FixedUpdateVector3SmoothValue,
}
#[cfg(feature = "SliderController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderController => ""
    ."SliderController"
);
#[cfg(feature = "SliderController")]
impl std::ops::Deref for crate::GlobalNamespace::SliderController {
    type Target = crate::GlobalNamespace::SliderControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderController")]
impl crate::GlobalNamespace::SliderController {
    pub const kSaberAttractPointNormalizedPosition: f32 = 0.7f32;
    #[cfg(feature = "SliderController+LengthType")]
    pub type LengthType = crate::GlobalNamespace::SliderController_LengthType;
    #[cfg(feature = "SliderController+Pool")]
    pub type Pool = crate::GlobalNamespace::SliderController_Pool;
    #[cfg(feature = "SliderController+_DissolveCoroutine_d__70")]
    pub type _DissolveCoroutine_d__70 = crate::GlobalNamespace::SliderController__DissolveCoroutine_d__70;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dissolve(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dissolve", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn DissolveCoroutine(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("DissolveCoroutine", (duration))?;
        Ok(__cordl_ret)
    }
    pub fn FixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleFadeInDidStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFadeInDidStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleHeadDidMovePastCutMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHeadDidMovePastCutMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleMovementDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMovementDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
        noteCutInfo: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::NoteCutInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (noteController, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        noteController: *mut crate::GlobalNamespace::NoteController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn HandleTailDidMovePastCutMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTailDidMovePastCutMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        lengthType: crate::GlobalNamespace::SliderController_LengthType,
        sliderData: *mut crate::GlobalNamespace::SliderData,
        worldRotation: f32,
        headNoteJumpStartPos: crate::UnityEngine::Vector3,
        tailNoteJumpStartPos: crate::UnityEngine::Vector3,
        headNoteJumpEndPos: crate::UnityEngine::Vector3,
        tailNoteJumpEndPos: crate::UnityEngine::Vector3,
        jumpDuration: f32,
        startNoteJumpGravity: f32,
        endNoteJumpGravity: f32,
        noteUniformScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    lengthType,
                    sliderData,
                    worldRotation,
                    headNoteJumpStartPos,
                    tailNoteJumpStartPos,
                    headNoteJumpEndPos,
                    tailNoteJumpEndPos,
                    jumpDuration,
                    startNoteJumpGravity,
                    endNoteJumpGravity,
                    noteUniformScale,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsNoteStartOfThisSlider(
        &mut self,
        noteData: *mut crate::GlobalNamespace::NoteData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNoteStartOfThisSlider", (noteData))?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Pause(
        &mut self,
        pause: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", (pause))?;
        Ok(__cordl_ret)
    }
    pub fn SetSaberAttraction(
        &mut self,
        saberAttraction: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSaberAttraction", (saberAttraction))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMaterialPropertyBlock(
        &mut self,
        timeSinceHeadNoteJump: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaterialPropertyBlock", (timeSinceHeadNoteJump))?;
        Ok(__cordl_ret)
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
    pub fn get_closeSmoothedSaberInteractionPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::FixedUpdateVector3SmoothValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::FixedUpdateVector3SmoothValue = __cordl_object
            .invoke("get_closeSmoothedSaberInteractionPos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headJumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_headJumpOffsetY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_initColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_initColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_jumpDistance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpDistance", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lengthType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SliderController_LengthType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SliderController_LengthType = __cordl_object
            .invoke("get_lengthType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_randomValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_randomValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_saberInteractionParam(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_saberInteractionParam", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::SliderData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SliderData = __cordl_object
            .invoke("get_sliderData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderDidDissolveEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidDissolveEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidDissolveEvent,
        > = __cordl_object.invoke("get_sliderDidDissolveEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderDidFinishJumpEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidFinishJumpEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidFinishJumpEvent,
        > = __cordl_object.invoke("get_sliderDidFinishJumpEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderDidStartDissolvingEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidStartDissolvingEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderDidStartDissolvingEvent,
        > = __cordl_object.invoke("get_sliderDidStartDissolvingEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sliderDuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderHeadDidMovePastCutMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderHeadDidMovePastCutMarkEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderHeadDidMovePastCutMarkEvent,
        > = __cordl_object.invoke("get_sliderHeadDidMovePastCutMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderIntensityEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SliderIntensityEffect,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SliderIntensityEffect = __cordl_object
            .invoke("get_sliderIntensityEffect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderMeshController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::SliderMeshController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SliderMeshController = __cordl_object
            .invoke("get_sliderMeshController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::SliderMovement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SliderMovement = __cordl_object
            .invoke("get_sliderMovement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderTailDidMovePastCutMark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ILazyCopyHashSet_1<
            *mut crate::GlobalNamespace::ISliderTailDidMovePastCutMarkEvent,
        > = __cordl_object.invoke("get_sliderTailDidMovePastCutMark", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_zDistanceBetweenNotes(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_zDistanceBetweenNotes", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SliderController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SliderController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

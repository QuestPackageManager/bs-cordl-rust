#[cfg(feature = "SliderIntensityEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderIntensityEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _longSliderHeadIntensity: f32,
    pub _shortSliderHeadIntensity: f32,
    pub _tailIntensity: f32,
    pub _fadeOutDuration: f32,
    pub _stayOffDuration: f32,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderIntensityEffect_InitData,
    >,
    pub fadeInDidStartEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _coreIntensity: f32,
    pub _effectIntensity: f32,
    pub _sliderDuration: f32,
    pub _headIntensity: f32,
    pub _intensityCalculationDelegate: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate,
    >,
    pub _dipEffectFadeElements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SliderIntensityEffect_FadeElement,
        >,
    >,
    pub _fadeInEffectFadeElements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SliderIntensityEffect_FadeElement,
        >,
    >,
}
#[cfg(feature = "SliderIntensityEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderIntensityEffect => ""
    ."SliderIntensityEffect"
);
#[cfg(feature = "SliderIntensityEffect")]
impl std::ops::Deref for crate::GlobalNamespace::SliderIntensityEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderIntensityEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect")]
impl crate::GlobalNamespace::SliderIntensityEffect {
    #[cfg(feature = "SliderIntensityEffect+FadeElement")]
    pub type FadeElement = crate::GlobalNamespace::SliderIntensityEffect_FadeElement;
    #[cfg(feature = "SliderIntensityEffect+InitData")]
    pub type InitData = crate::GlobalNamespace::SliderIntensityEffect_InitData;
    #[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
    pub type IntensityCalculationDelegate = crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        sliderDuration: f32,
        initialHalfJumpDuration: f32,
        startVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (sliderDuration, initialHalfJumpDuration, startVisible))?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        timeSinceHeadNoteJump: f32,
        halfJumpDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (timeSinceHeadNoteJump, halfJumpDuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessEffectCoroutine(
        &mut self,
        fadeElements: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SliderIntensityEffect_FadeElement,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("ProcessEffectCoroutine", (fadeElements))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartIntensityDipEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartIntensityDipEffect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartIntensityFadeInEffect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartIntensityFadeInEffect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__24_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__24_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__24_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__24_1", ())?;
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
    pub fn add_fadeInDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_fadeInDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_colorIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_intensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_intensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_fadeInDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_fadeInDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderIntensityEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderIntensityEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderIntensityEffect_FadeElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub duration: f32,
    pub startIntensity: f32,
    pub endIntensity: f32,
    pub easeType: crate::GlobalNamespace::EaseType,
    pub startCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SliderIntensityEffect_FadeElement => ""
    ."SliderIntensityEffect/FadeElement"
);
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
impl std::ops::Deref for crate::GlobalNamespace::SliderIntensityEffect_FadeElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderIntensityEffect_FadeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
impl crate::GlobalNamespace::SliderIntensityEffect_FadeElement {
    pub fn New(
        easeType: crate::GlobalNamespace::EaseType,
        startIntensity: f32,
        endIntensity: f32,
        startCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (easeType, startIntensity, endIntensity, startCallback),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        easeType: crate::GlobalNamespace::EaseType,
        startIntensity: f32,
        endIntensity: f32,
        startCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (easeType, startIntensity, endIntensity, startCallback))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderIntensityEffect+FadeElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderIntensityEffect_FadeElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderIntensityEffect+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderIntensityEffect_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sliderColorIntensity: f32,
    pub hapticFeedback: f32,
}
#[cfg(feature = "SliderIntensityEffect+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderIntensityEffect_InitData
    => ""."SliderIntensityEffect/InitData"
);
#[cfg(feature = "SliderIntensityEffect+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::SliderIntensityEffect_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SliderIntensityEffect_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+InitData")]
impl crate::GlobalNamespace::SliderIntensityEffect_InitData {
    pub fn New(
        arcVisibilityType: crate::GlobalNamespace::ArcVisibilityType,
        hapticFeedbackEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (arcVisibilityType, hapticFeedbackEnabled))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        arcVisibilityType: crate::GlobalNamespace::ArcVisibilityType,
        hapticFeedbackEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (arcVisibilityType, hapticFeedbackEnabled))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SliderIntensityEffect+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderIntensityEffect_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderIntensityEffect_IntensityCalculationDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate => ""
    ."SliderIntensityEffect/IntensityCalculationDelegate"
);
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
impl crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate {
    pub fn BeginInvoke(
        &mut self,
        timeSinceLastSection: f32,
        timeSinceHeadNoteJump: f32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (timeSinceLastSection, timeSinceHeadNoteJump, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        timeSinceLastSection: f32,
        timeSinceHeadNoteJump: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("Invoke", (timeSinceLastSection, timeSinceHeadNoteJump))?;
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
#[cfg(feature = "SliderIntensityEffect+IntensityCalculationDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SliderIntensityEffect_IntensityCalculationDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

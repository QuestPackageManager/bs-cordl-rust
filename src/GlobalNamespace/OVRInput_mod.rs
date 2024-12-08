#[cfg(feature = "OVRInput+Axis1D")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Axis1D {
    Any = -1i32,
    None = 0i32,
    PrimaryHandTrigger = 4i32,
    PrimaryIndexTrigger = 1i32,
    PrimaryIndexTriggerCurl = 16i32,
    PrimaryIndexTriggerForce = 4096i32,
    PrimaryIndexTriggerSlide = 32i32,
    PrimaryStylusForce = 128i32,
    PrimaryThumbRestForce = 64i32,
    SecondaryHandTrigger = 8i32,
    SecondaryIndexTrigger = 2i32,
    SecondaryIndexTriggerCurl = 256i32,
    SecondaryIndexTriggerForce = 8192i32,
    SecondaryIndexTriggerSlide = 512i32,
    SecondaryStylusForce = 2048i32,
    SecondaryThumbRestForce = 1024i32,
}
#[cfg(feature = "OVRInput+Axis1D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Axis1D => ""
    ."OVRInput/Axis1D"
);
#[cfg(feature = "OVRInput+Axis2D")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Axis2D {
    Any = -1i32,
    None = 0i32,
    PrimaryThumbstick = 1i32,
    PrimaryTouchpad = 4i32,
    SecondaryThumbstick = 2i32,
    SecondaryTouchpad = 8i32,
}
#[cfg(feature = "OVRInput+Axis2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Axis2D => ""
    ."OVRInput/Axis2D"
);
#[cfg(feature = "OVRInput+Button")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Button {
    Any = -1i32,
    Back = 512i32,
    Down = 536870912i32,
    DpadDown = 32i32,
    DpadLeft = 64i32,
    DpadRight = 128i32,
    DpadUp = 16i32,
    Four = 8i32,
    Left = 1073741824i32,
    None = 0i32,
    One = 1i32,
    PrimaryHandTrigger = 16384i32,
    PrimaryIndexTrigger = 8192i32,
    PrimaryShoulder = 4096i32,
    PrimaryThumbstick = 32768i32,
    PrimaryThumbstickDown = 131072i32,
    PrimaryThumbstickLeft = 262144i32,
    PrimaryThumbstickRight = 524288i32,
    PrimaryThumbstickUp = 65536i32,
    PrimaryTouchpad = 1024i32,
    Right = -2147483648i32,
    SecondaryHandTrigger = 4194304i32,
    SecondaryIndexTrigger = 2097152i32,
    SecondaryShoulder = 1048576i32,
    SecondaryThumbstick = 8388608i32,
    SecondaryThumbstickDown = 33554432i32,
    SecondaryThumbstickLeft = 67108864i32,
    SecondaryThumbstickRight = 134217728i32,
    SecondaryThumbstickUp = 16777216i32,
    SecondaryTouchpad = 2048i32,
    Start = 256i32,
    Three = 4i32,
    Two = 2i32,
    Up = 268435456i32,
}
#[cfg(feature = "OVRInput+Button")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Button => ""
    ."OVRInput/Button"
);
#[cfg(feature = "OVRInput+Controller")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Controller {
    Active = -2147483648i32,
    All = -1i32,
    Gamepad = 16i32,
    Hands = 96i32,
    LHand = 32i32,
    LTouch = 1i32,
    None = 0i32,
    RHand = 64i32,
    RTouch = 2i32,
    Remote = 4i32,
    Touch = 3i32,
}
#[cfg(feature = "OVRInput+Controller")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Controller => ""
    ."OVRInput/Controller"
);
#[cfg(feature = "OVRInput+ControllerInHandState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_ControllerInHandState {
    ControllerInHand = 1i32,
    ControllerNotInHand = 2i32,
    NoHand = 0i32,
}
#[cfg(feature = "OVRInput+ControllerInHandState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_ControllerInHandState
    => ""."OVRInput/ControllerInHandState"
);
#[cfg(feature = "OVRInput+Hand")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Hand {
    HandLeft = 0i32,
    HandRight = 1i32,
    None = -1i32,
}
#[cfg(feature = "OVRInput+Hand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Hand => ""
    ."OVRInput/Hand"
);
#[cfg(feature = "OVRInput+Handedness")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Handedness {
    LeftHanded = 1i32,
    RightHanded = 2i32,
    Unsupported = 0i32,
}
#[cfg(feature = "OVRInput+Handedness")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Handedness => ""
    ."OVRInput/Handedness"
);
#[cfg(feature = "OVRInput+HapticInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_HapticInfo {
    __cordl_parent: crate::System::Object,
    pub playingHaptics: bool,
    pub hapticsDurationPlayed: f32,
    pub hapticsDuration: f32,
    pub hapticAmplitude: f32,
    pub node: crate::UnityEngine::XR::XRNode,
}
#[cfg(feature = "OVRInput+HapticInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_HapticInfo => ""
    ."OVRInput/HapticInfo"
);
#[cfg(feature = "OVRInput+HapticInfo")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_HapticInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+HapticInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_HapticInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+HapticInfo")]
impl crate::GlobalNamespace::OVRInput_HapticInfo {
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
#[cfg(feature = "OVRInput+HapticInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRInput_HapticInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+HapticsAmplitudeEnvelopeVibration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRInput_HapticsAmplitudeEnvelopeVibration {
    pub SamplesCount: i32,
    pub Samples: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub Duration: f32,
}
#[cfg(feature = "OVRInput+HapticsAmplitudeEnvelopeVibration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRInput_HapticsAmplitudeEnvelopeVibration => ""
    ."OVRInput/HapticsAmplitudeEnvelopeVibration"
);
#[cfg(feature = "OVRInput+HapticsAmplitudeEnvelopeVibration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRInput_HapticsAmplitudeEnvelopeVibration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRInput+HapticsAmplitudeEnvelopeVibration")]
impl crate::GlobalNamespace::OVRInput_HapticsAmplitudeEnvelopeVibration {}
#[cfg(feature = "OVRInput+HapticsLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_HapticsLocation {
    Hand = 1i32,
    Index = 4i32,
    None = 0i32,
    Thumb = 2i32,
}
#[cfg(feature = "OVRInput+HapticsLocation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_HapticsLocation => ""
    ."OVRInput/HapticsLocation"
);
#[cfg(feature = "OVRInput+HapticsPcmVibration")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRInput_HapticsPcmVibration {
    pub SamplesCount: i32,
    pub Samples: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub SampleRateHz: f32,
    pub Append: bool,
}
#[cfg(feature = "OVRInput+HapticsPcmVibration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_HapticsPcmVibration =>
    ""."OVRInput/HapticsPcmVibration"
);
#[cfg(feature = "OVRInput+HapticsPcmVibration")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRInput_HapticsPcmVibration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRInput+HapticsPcmVibration")]
impl crate::GlobalNamespace::OVRInput_HapticsPcmVibration {}
#[cfg(feature = "OVRInput+InputDeviceShowState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_InputDeviceShowState {
    Always = 0i32,
    ControllerInHand = 2i32,
    ControllerInHandOrNoHand = 1i32,
    ControllerNotInHand = 3i32,
    NoHand = 4i32,
}
#[cfg(feature = "OVRInput+InputDeviceShowState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_InputDeviceShowState
    => ""."OVRInput/InputDeviceShowState"
);
#[cfg(feature = "OVRInput+InteractionProfile")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_InteractionProfile {
    None = 0i32,
    Touch = 1i32,
    TouchPlus = 4i32,
    TouchPro = 2i32,
}
#[cfg(feature = "OVRInput+InteractionProfile")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_InteractionProfile =>
    ""."OVRInput/InteractionProfile"
);
#[cfg(feature = "OVRInput+NearTouch")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_NearTouch {
    Any = -1i32,
    None = 0i32,
    PrimaryIndexTrigger = 1i32,
    PrimaryThumbButtons = 2i32,
    SecondaryIndexTrigger = 4i32,
    SecondaryThumbButtons = 8i32,
}
#[cfg(feature = "OVRInput+NearTouch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_NearTouch => ""
    ."OVRInput/NearTouch"
);
#[cfg(feature = "OVRInput+OVRControllerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerBase {
    __cordl_parent: crate::System::Object,
    pub controllerType: crate::GlobalNamespace::OVRInput_Controller,
    pub buttonMap: *mut crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap,
    pub touchMap: *mut crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap,
    pub nearTouchMap: *mut crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap,
    pub axis1DMap: *mut crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap,
    pub axis2DMap: *mut crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap,
    pub previousState: crate::GlobalNamespace::OVRPlugin_ControllerState6,
    pub currentState: crate::GlobalNamespace::OVRPlugin_ControllerState6,
    pub shouldApplyDeadzone: bool,
    pub HapticsPcmSamplesConsumedCache: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "OVRInput+OVRControllerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerBase =>
    ""."OVRInput/OVRControllerBase"
);
#[cfg(feature = "OVRInput+OVRControllerBase")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase")]
impl crate::GlobalNamespace::OVRInput_OVRControllerBase {
    #[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
    pub type VirtualTouchMap = crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap;
    #[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
    pub type VirtualAxis1DMap = crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap;
    #[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
    pub type VirtualAxis2DMap = crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap;
    #[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
    pub type VirtualButtonMap = crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap;
    #[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
    pub type VirtualNearTouchMap = crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap;
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetControllerSampleRateHz(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetControllerSampleRateHz", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOpenVRControllerState(
        &mut self,
        controllerType: crate::GlobalNamespace::OVRInput_Controller,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_ControllerState6,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_ControllerState6 = __cordl_object
            .invoke("GetOpenVRControllerState", (controllerType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResolveToRawMask_OVRInput_Axis1D3(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Axis1D,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawAxis1D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawAxis1D = __cordl_object
            .invoke("ResolveToRawMask", (virtualMask))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveToRawMask_OVRInput_Axis2D4(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Axis2D,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawAxis2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawAxis2D = __cordl_object
            .invoke("ResolveToRawMask", (virtualMask))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveToRawMask_OVRInput_Button0(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Button,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawButton> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawButton = __cordl_object
            .invoke("ResolveToRawMask", (virtualMask))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveToRawMask_OVRInput_NearTouch2(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_NearTouch,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawNearTouch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawNearTouch = __cordl_object
            .invoke("ResolveToRawMask", (virtualMask))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveToRawMask_OVRInput_Touch1(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Touch,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawTouch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawTouch = __cordl_object
            .invoke("ResolveToRawMask", (virtualMask))?;
        Ok(__cordl_ret)
    }
    pub fn SetControllerHapticsAmplitudeEnvelope(
        &mut self,
        hapticsVibration: crate::GlobalNamespace::OVRInput_HapticsAmplitudeEnvelopeVibration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetControllerHapticsAmplitudeEnvelope", (hapticsVibration))?;
        Ok(__cordl_ret)
    }
    pub fn SetControllerHapticsPcm(
        &mut self,
        hapticsVibration: crate::GlobalNamespace::OVRInput_HapticsPcmVibration,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SetControllerHapticsPcm", (hapticsVibration))?;
        Ok(__cordl_ret)
    }
    pub fn SetControllerLocalizedVibration(
        &mut self,
        hapticsLocationMask: crate::GlobalNamespace::OVRInput_HapticsLocation,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetControllerLocalizedVibration",
                (hapticsLocationMask, frequency, amplitude),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetControllerVibration(
        &mut self,
        frequency: f32,
        amplitude: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetControllerVibration", (frequency, amplitude))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_Controller> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_Controller = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerGamepadAndroid {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid => ""
    ."OVRInput/OVRControllerGamepadAndroid"
);
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
impl crate::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerGamepadPC {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerGamepadPC
    => ""."OVRInput/OVRControllerGamepadPC"
);
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerGamepadPC {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerGamepadPC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
impl crate::GlobalNamespace::OVRInput_OVRControllerGamepadPC {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerGamepadPC {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerHands")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerHands {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerHands")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerHands =>
    ""."OVRInput/OVRControllerHands"
);
#[cfg(feature = "OVRInput+OVRControllerHands")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerHands {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerHands")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerHands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerHands")]
impl crate::GlobalNamespace::OVRInput_OVRControllerHands {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerHands")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerHands {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerLHand")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerLHand {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerLHand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerLHand =>
    ""."OVRInput/OVRControllerLHand"
);
#[cfg(feature = "OVRInput+OVRControllerLHand")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerLHand {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerLHand")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerLHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerLHand")]
impl crate::GlobalNamespace::OVRInput_OVRControllerLHand {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerLHand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerLHand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerLTouch {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerLTouch =>
    ""."OVRInput/OVRControllerLTouch"
);
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerLTouch {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerLTouch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
impl crate::GlobalNamespace::OVRInput_OVRControllerLTouch {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerLTouch")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerLTouch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerRHand")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerRHand {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerRHand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerRHand =>
    ""."OVRInput/OVRControllerRHand"
);
#[cfg(feature = "OVRInput+OVRControllerRHand")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerRHand {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRHand")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerRHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRHand")]
impl crate::GlobalNamespace::OVRInput_OVRControllerRHand {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerRHand")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerRHand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerRTouch {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerRTouch =>
    ""."OVRInput/OVRControllerRTouch"
);
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerRTouch {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerRTouch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
impl crate::GlobalNamespace::OVRInput_OVRControllerRTouch {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerRTouch")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerRTouch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerRemote")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerRemote {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerRemote")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerRemote =>
    ""."OVRInput/OVRControllerRemote"
);
#[cfg(feature = "OVRInput+OVRControllerRemote")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerRemote {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRemote")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerRemote {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerRemote")]
impl crate::GlobalNamespace::OVRInput_OVRControllerRemote {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerRemote")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerRemote {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerTouch")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput_OVRControllerTouch {
    __cordl_parent: crate::GlobalNamespace::OVRInput_OVRControllerBase,
}
#[cfg(feature = "OVRInput+OVRControllerTouch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OVRControllerTouch =>
    ""."OVRInput/OVRControllerTouch"
);
#[cfg(feature = "OVRInput+OVRControllerTouch")]
impl std::ops::Deref for crate::GlobalNamespace::OVRInput_OVRControllerTouch {
    type Target = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerTouch")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRInput_OVRControllerTouch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerTouch")]
impl crate::GlobalNamespace::OVRInput_OVRControllerTouch {
    pub fn ConfigureAxis1DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis1DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureAxis2DMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureAxis2DMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureButtonMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureButtonMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureNearTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureNearTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConfigureTouchMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureTouchMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBatteryPercentRemaining(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetBatteryPercentRemaining", ())?;
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
}
#[cfg(feature = "OVRInput+OVRControllerTouch")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRInput_OVRControllerTouch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInput {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRInput => ""."OVRInput"
);
#[cfg(feature = "OVRInput")]
impl std::ops::Deref for OVRInput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput")]
impl std::ops::DerefMut for OVRInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput")]
impl OVRInput {
    #[cfg(feature = "OVRInput+Touch")]
    pub type Touch = crate::GlobalNamespace::OVRInput_Touch;
    #[cfg(feature = "OVRInput+HapticsAmplitudeEnvelopeVibration")]
    pub type HapticsAmplitudeEnvelopeVibration = crate::GlobalNamespace::OVRInput_HapticsAmplitudeEnvelopeVibration;
    #[cfg(feature = "OVRInput+RawButton")]
    pub type RawButton = crate::GlobalNamespace::OVRInput_RawButton;
    #[cfg(feature = "OVRInput+OVRControllerHands")]
    pub type OVRControllerHands = crate::GlobalNamespace::OVRInput_OVRControllerHands;
    #[cfg(feature = "OVRInput+Hand")]
    pub type Hand = crate::GlobalNamespace::OVRInput_Hand;
    #[cfg(feature = "OVRInput+OVRControllerGamepadPC")]
    pub type OVRControllerGamepadPC = crate::GlobalNamespace::OVRInput_OVRControllerGamepadPC;
    #[cfg(feature = "OVRInput+RawAxis1D")]
    pub type RawAxis1D = crate::GlobalNamespace::OVRInput_RawAxis1D;
    #[cfg(feature = "OVRInput+OpenVRController")]
    pub type OpenVRController = crate::GlobalNamespace::OVRInput_OpenVRController;
    #[cfg(feature = "OVRInput+HapticInfo")]
    pub type HapticInfo = crate::GlobalNamespace::OVRInput_HapticInfo;
    #[cfg(feature = "OVRInput+OVRControllerTouch")]
    pub type OVRControllerTouch = crate::GlobalNamespace::OVRInput_OVRControllerTouch;
    #[cfg(feature = "OVRInput+OVRControllerLTouch")]
    pub type OVRControllerLTouch = crate::GlobalNamespace::OVRInput_OVRControllerLTouch;
    #[cfg(feature = "OVRInput+OVRControllerBase")]
    pub type OVRControllerBase = crate::GlobalNamespace::OVRInput_OVRControllerBase;
    #[cfg(feature = "OVRInput+OVRControllerRTouch")]
    pub type OVRControllerRTouch = crate::GlobalNamespace::OVRInput_OVRControllerRTouch;
    #[cfg(feature = "OVRInput+OVRControllerLHand")]
    pub type OVRControllerLHand = crate::GlobalNamespace::OVRInput_OVRControllerLHand;
    #[cfg(feature = "OVRInput+OpenVRButton")]
    pub type OpenVRButton = crate::GlobalNamespace::OVRInput_OpenVRButton;
    #[cfg(feature = "OVRInput+OVRControllerRHand")]
    pub type OVRControllerRHand = crate::GlobalNamespace::OVRInput_OVRControllerRHand;
    #[cfg(feature = "OVRInput+OVRControllerRemote")]
    pub type OVRControllerRemote = crate::GlobalNamespace::OVRInput_OVRControllerRemote;
    #[cfg(feature = "OVRInput+OVRControllerGamepadAndroid")]
    pub type OVRControllerGamepadAndroid = crate::GlobalNamespace::OVRInput_OVRControllerGamepadAndroid;
    #[cfg(feature = "OVRInput+NearTouch")]
    pub type NearTouch = crate::GlobalNamespace::OVRInput_NearTouch;
    #[cfg(feature = "OVRInput+InteractionProfile")]
    pub type InteractionProfile = crate::GlobalNamespace::OVRInput_InteractionProfile;
    #[cfg(feature = "OVRInput+Axis2D")]
    pub type Axis2D = crate::GlobalNamespace::OVRInput_Axis2D;
    #[cfg(feature = "OVRInput+Button")]
    pub type Button = crate::GlobalNamespace::OVRInput_Button;
    #[cfg(feature = "OVRInput+Controller")]
    pub type Controller = crate::GlobalNamespace::OVRInput_Controller;
    #[cfg(feature = "OVRInput+Handedness")]
    pub type Handedness = crate::GlobalNamespace::OVRInput_Handedness;
    #[cfg(feature = "OVRInput+HapticsLocation")]
    pub type HapticsLocation = crate::GlobalNamespace::OVRInput_HapticsLocation;
    #[cfg(feature = "OVRInput+ControllerInHandState")]
    pub type ControllerInHandState = crate::GlobalNamespace::OVRInput_ControllerInHandState;
    #[cfg(feature = "OVRInput+OpenVRControllerDetails")]
    pub type OpenVRControllerDetails = crate::GlobalNamespace::OVRInput_OpenVRControllerDetails;
    #[cfg(feature = "OVRInput+Axis1D")]
    pub type Axis1D = crate::GlobalNamespace::OVRInput_Axis1D;
    #[cfg(feature = "OVRInput+RawTouch")]
    pub type RawTouch = crate::GlobalNamespace::OVRInput_RawTouch;
    #[cfg(feature = "OVRInput+RawNearTouch")]
    pub type RawNearTouch = crate::GlobalNamespace::OVRInput_RawNearTouch;
    #[cfg(feature = "OVRInput+RawAxis2D")]
    pub type RawAxis2D = crate::GlobalNamespace::OVRInput_RawAxis2D;
    #[cfg(feature = "OVRInput+HapticsPcmVibration")]
    pub type HapticsPcmVibration = crate::GlobalNamespace::OVRInput_HapticsPcmVibration;
    #[cfg(feature = "OVRInput+InputDeviceShowState")]
    pub type InputDeviceShowState = crate::GlobalNamespace::OVRInput_InputDeviceShowState;
}
#[cfg(feature = "OVRInput")]
impl quest_hook::libil2cpp::ObjectType for OVRInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OpenVRButton")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_OpenVRButton {
    Grip = 4u64,
    None = 0u64,
    Thumbstick = 4294967296u64,
    Two = 2u64,
}
#[cfg(feature = "OVRInput+OpenVRButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OpenVRButton => ""
    ."OVRInput/OpenVRButton"
);
#[cfg(feature = "OVRInput+OpenVRController")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_OpenVRController {
    OculusTouch = 1u64,
    Unknown = 0u64,
    ViveController = 2u64,
    WindowsMRController = 3u64,
}
#[cfg(feature = "OVRInput+OpenVRController")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_OpenVRController => ""
    ."OVRInput/OpenVRController"
);
#[cfg(feature = "OVRInput+OpenVRControllerDetails")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRInput_OpenVRControllerDetails {
    pub state: crate::OVR::OpenVR::VRControllerState_t,
    pub controllerType: crate::GlobalNamespace::OVRInput_OpenVRController,
    pub deviceID: u32,
    pub localPosition: crate::UnityEngine::Vector3,
    pub localOrientation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "OVRInput+OpenVRControllerDetails")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRInput_OpenVRControllerDetails => ""
    ."OVRInput/OpenVRControllerDetails"
);
#[cfg(feature = "OVRInput+OpenVRControllerDetails")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRInput_OpenVRControllerDetails {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRInput+OpenVRControllerDetails")]
impl crate::GlobalNamespace::OVRInput_OpenVRControllerDetails {}
#[cfg(feature = "OVRInput+RawAxis1D")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_RawAxis1D {
    Any = -1i32,
    LHandTrigger = 4i32,
    LIndexTrigger = 1i32,
    LIndexTriggerCurl = 16i32,
    LIndexTriggerForce = 4096i32,
    LIndexTriggerSlide = 32i32,
    LStylusForce = 128i32,
    LThumbRestForce = 64i32,
    None = 0i32,
    RHandTrigger = 8i32,
    RIndexTrigger = 2i32,
    RIndexTriggerCurl = 256i32,
    RIndexTriggerForce = 8192i32,
    RIndexTriggerSlide = 512i32,
    RStylusForce = 2048i32,
    RThumbRestForce = 1024i32,
}
#[cfg(feature = "OVRInput+RawAxis1D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_RawAxis1D => ""
    ."OVRInput/RawAxis1D"
);
#[cfg(feature = "OVRInput+RawAxis2D")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_RawAxis2D {
    Any = -1i32,
    LThumbstick = 1i32,
    LTouchpad = 4i32,
    None = 0i32,
    RThumbstick = 2i32,
    RTouchpad = 8i32,
}
#[cfg(feature = "OVRInput+RawAxis2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_RawAxis2D => ""
    ."OVRInput/RawAxis2D"
);
#[cfg(feature = "OVRInput+RawButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_RawButton {
    A = 1i32,
    Any = -1i32,
    B = 2i32,
    Back = 2097152i32,
    DpadDown = 131072i32,
    DpadLeft = 262144i32,
    DpadRight = 524288i32,
    DpadUp = 65536i32,
    LHandTrigger = 536870912i32,
    LIndexTrigger = 268435456i32,
    LShoulder = 2048i32,
    LThumbstick = 1024i32,
    LThumbstickDown = 32i32,
    LThumbstickLeft = 64i32,
    LThumbstickRight = 128i32,
    LThumbstickUp = 16i32,
    LTouchpad = 1073741824i32,
    None = 0i32,
    RHandTrigger = 134217728i32,
    RIndexTrigger = 67108864i32,
    RShoulder = 8i32,
    RThumbstick = 4i32,
    RThumbstickDown = 8192i32,
    RThumbstickLeft = 16384i32,
    RThumbstickRight = 32768i32,
    RThumbstickUp = 4096i32,
    RTouchpad = -2147483648i32,
    Start = 1048576i32,
    X = 256i32,
    Y = 512i32,
}
#[cfg(feature = "OVRInput+RawButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_RawButton => ""
    ."OVRInput/RawButton"
);
#[cfg(feature = "OVRInput+RawNearTouch")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_RawNearTouch {
    Any = -1i32,
    LIndexTrigger = 1i32,
    LThumbButtons = 2i32,
    None = 0i32,
    RIndexTrigger = 4i32,
    RThumbButtons = 8i32,
}
#[cfg(feature = "OVRInput+RawNearTouch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_RawNearTouch => ""
    ."OVRInput/RawNearTouch"
);
#[cfg(feature = "OVRInput+RawTouch")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_RawTouch {
    A = 1i32,
    Any = -1i32,
    B = 2i32,
    LIndexTrigger = 4096i32,
    LThumbRest = 2048i32,
    LThumbstick = 1024i32,
    LTouchpad = 1073741824i32,
    None = 0i32,
    RIndexTrigger = 16i32,
    RThumbRest = 8i32,
    RThumbstick = 4i32,
    RTouchpad = -2147483648i32,
    X = 256i32,
    Y = 512i32,
}
#[cfg(feature = "OVRInput+RawTouch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_RawTouch => ""
    ."OVRInput/RawTouch"
);
#[cfg(feature = "OVRInput+Touch")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInput_Touch {
    Any = -1i32,
    Four = 8i32,
    None = 0i32,
    One = 1i32,
    PrimaryIndexTrigger = 8192i32,
    PrimaryThumbRest = 4096i32,
    PrimaryThumbstick = 32768i32,
    PrimaryTouchpad = 1024i32,
    SecondaryIndexTrigger = 2097152i32,
    SecondaryThumbRest = 1048576i32,
    SecondaryThumbstick = 8388608i32,
    SecondaryTouchpad = 2048i32,
    Three = 4i32,
    Two = 2i32,
}
#[cfg(feature = "OVRInput+Touch")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRInput_Touch => ""
    ."OVRInput/Touch"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerBase_VirtualAxis1DMap {
    __cordl_parent: crate::System::Object,
    pub None: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryHandTrigger: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryHandTrigger: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryIndexTriggerCurl: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryIndexTriggerSlide: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryThumbRestForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryStylusForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryIndexTriggerCurl: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryIndexTriggerSlide: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryThumbRestForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryStylusForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub PrimaryIndexTriggerForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
    pub SecondaryIndexTriggerForce: crate::GlobalNamespace::OVRInput_RawAxis1D,
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap => ""
    ."OVRInput/OVRControllerBase/VirtualAxis1DMap"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
impl crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToRawMask(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Axis1D,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawAxis1D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawAxis1D = __cordl_object
            .invoke("ToRawMask", (virtualMask))?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis1DMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerBase_VirtualAxis1DMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerBase_VirtualAxis2DMap {
    __cordl_parent: crate::System::Object,
    pub None: crate::GlobalNamespace::OVRInput_RawAxis2D,
    pub PrimaryThumbstick: crate::GlobalNamespace::OVRInput_RawAxis2D,
    pub PrimaryTouchpad: crate::GlobalNamespace::OVRInput_RawAxis2D,
    pub SecondaryThumbstick: crate::GlobalNamespace::OVRInput_RawAxis2D,
    pub SecondaryTouchpad: crate::GlobalNamespace::OVRInput_RawAxis2D,
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap => ""
    ."OVRInput/OVRControllerBase/VirtualAxis2DMap"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
impl crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToRawMask(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Axis2D,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawAxis2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawAxis2D = __cordl_object
            .invoke("ToRawMask", (virtualMask))?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualAxis2DMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerBase_VirtualAxis2DMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerBase_VirtualButtonMap {
    __cordl_parent: crate::System::Object,
    pub None: crate::GlobalNamespace::OVRInput_RawButton,
    pub One: crate::GlobalNamespace::OVRInput_RawButton,
    pub Two: crate::GlobalNamespace::OVRInput_RawButton,
    pub Three: crate::GlobalNamespace::OVRInput_RawButton,
    pub Four: crate::GlobalNamespace::OVRInput_RawButton,
    pub Start: crate::GlobalNamespace::OVRInput_RawButton,
    pub Back: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryShoulder: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryHandTrigger: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryThumbstick: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryThumbstickUp: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryThumbstickDown: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryThumbstickLeft: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryThumbstickRight: crate::GlobalNamespace::OVRInput_RawButton,
    pub PrimaryTouchpad: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryShoulder: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryHandTrigger: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryThumbstick: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryThumbstickUp: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryThumbstickDown: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryThumbstickLeft: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryThumbstickRight: crate::GlobalNamespace::OVRInput_RawButton,
    pub SecondaryTouchpad: crate::GlobalNamespace::OVRInput_RawButton,
    pub DpadUp: crate::GlobalNamespace::OVRInput_RawButton,
    pub DpadDown: crate::GlobalNamespace::OVRInput_RawButton,
    pub DpadLeft: crate::GlobalNamespace::OVRInput_RawButton,
    pub DpadRight: crate::GlobalNamespace::OVRInput_RawButton,
    pub Up: crate::GlobalNamespace::OVRInput_RawButton,
    pub Down: crate::GlobalNamespace::OVRInput_RawButton,
    pub Left: crate::GlobalNamespace::OVRInput_RawButton,
    pub Right: crate::GlobalNamespace::OVRInput_RawButton,
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerBase_VirtualButtonMap => ""
    ."OVRInput/OVRControllerBase/VirtualButtonMap"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
impl crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToRawMask(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Button,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawButton> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawButton = __cordl_object
            .invoke("ToRawMask", (virtualMask))?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualButtonMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerBase_VirtualButtonMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerBase_VirtualNearTouchMap {
    __cordl_parent: crate::System::Object,
    pub None: crate::GlobalNamespace::OVRInput_RawNearTouch,
    pub PrimaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawNearTouch,
    pub PrimaryThumbButtons: crate::GlobalNamespace::OVRInput_RawNearTouch,
    pub SecondaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawNearTouch,
    pub SecondaryThumbButtons: crate::GlobalNamespace::OVRInput_RawNearTouch,
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap => ""
    ."OVRInput/OVRControllerBase/VirtualNearTouchMap"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
impl crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToRawMask(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_NearTouch,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawNearTouch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawNearTouch = __cordl_object
            .invoke("ToRawMask", (virtualMask))?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualNearTouchMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerBase_VirtualNearTouchMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerBase_VirtualTouchMap {
    __cordl_parent: crate::System::Object,
    pub None: crate::GlobalNamespace::OVRInput_RawTouch,
    pub One: crate::GlobalNamespace::OVRInput_RawTouch,
    pub Two: crate::GlobalNamespace::OVRInput_RawTouch,
    pub Three: crate::GlobalNamespace::OVRInput_RawTouch,
    pub Four: crate::GlobalNamespace::OVRInput_RawTouch,
    pub PrimaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawTouch,
    pub PrimaryThumbstick: crate::GlobalNamespace::OVRInput_RawTouch,
    pub PrimaryThumbRest: crate::GlobalNamespace::OVRInput_RawTouch,
    pub PrimaryTouchpad: crate::GlobalNamespace::OVRInput_RawTouch,
    pub SecondaryIndexTrigger: crate::GlobalNamespace::OVRInput_RawTouch,
    pub SecondaryThumbstick: crate::GlobalNamespace::OVRInput_RawTouch,
    pub SecondaryThumbRest: crate::GlobalNamespace::OVRInput_RawTouch,
    pub SecondaryTouchpad: crate::GlobalNamespace::OVRInput_RawTouch,
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerBase_VirtualTouchMap => ""
    ."OVRInput/OVRControllerBase/VirtualTouchMap"
);
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
impl std::ops::Deref for crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
impl crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToRawMask(
        &mut self,
        virtualMask: crate::GlobalNamespace::OVRInput_Touch,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRInput_RawTouch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRInput_RawTouch = __cordl_object
            .invoke("ToRawMask", (virtualMask))?;
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
}
#[cfg(feature = "OVRInput+OVRControllerBase+VirtualTouchMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRControllerBase_VirtualTouchMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

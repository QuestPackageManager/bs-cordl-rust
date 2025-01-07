#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboardSampleInputHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub OVRVirtualKeyboard: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRVirtualKeyboard,
    >,
    pub raycaster: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRRaycaster>,
    pub inputModule: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::OVRInputModule,
    >,
    pub leftLinePointer: quest_hook::libil2cpp::Gc<crate::UnityEngine::LineRenderer>,
    pub rightLinePointer: quest_hook::libil2cpp::Gc<crate::UnityEngine::LineRenderer>,
    pub interactionDevice_: crate::System::Nullable_1<
        crate::GlobalNamespace::OVRInput_Controller,
    >,
    pub linePointerInitialWidth_: f32,
}
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboardSampleInputHandler";
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
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
impl crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler {
    pub const COLLISION_BOUNDS_ADDED_BLEED_PERCENT: f32 = 0.1f32;
    pub const LINEPOINTER_THINNING_THRESHOLD: f32 = 0.015f32;
    pub const RAY_MAX_DISTANCE: f32 = 100f32;
    pub const THUMBSTICK_DEADZONE: f32 = 0.2f32;
    pub fn ApplyDeadzone(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyDeadzone", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInteractionAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInteractionAnchor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLineRenderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLineRenderer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLineRendererFromSource(
        &mut self,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLineRendererFromSource", (source))?;
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
    pub fn get_AnalogStickX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AnalogStickX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AnalogStickY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AnalogStickY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputRayPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_InputRayPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InputRayRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_InputRayRotation", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboardSampleInputHandler {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub OVRVirtualKeyboard: *mut crate::GlobalNamespace::OVRVirtualKeyboard,
    pub raycaster: *mut crate::GlobalNamespace::OVRRaycaster,
    pub inputModule: *mut crate::UnityEngine::EventSystems::OVRInputModule,
    pub leftLinePointer: *mut crate::UnityEngine::LineRenderer,
    pub rightLinePointer: *mut crate::UnityEngine::LineRenderer,
    pub interactionDevice_: crate::System::Nullable_1<
        crate::GlobalNamespace::OVRInput_Controller,
    >,
    pub linePointerInitialWidth_: f32,
}
#[cfg(feature = "OVRVirtualKeyboardSampleInputHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler => ""
    ."OVRVirtualKeyboardSampleInputHandler"
);
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn UpdateInteractionAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInteractionAnchor", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLineRenderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLineRenderer", ())?;
        Ok(__cordl_ret)
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
    pub fn get_AnalogStickX(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AnalogStickX", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AnalogStickY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_AnalogStickY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputRayPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_InputRayPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InputRayRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_InputRayRotation", ())?;
        Ok(__cordl_ret)
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

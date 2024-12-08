#[cfg(feature = "CommandBufferBlurryScreenGrab")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBufferBlurryScreenGrab {
    __cordl_parent: CommandBufferGOCore,
    pub _kawaseBlurRenderer: *mut KawaseBlurRendererSO,
    pub _kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _cameraEvent: crate::UnityEngine::Rendering::CameraEvent,
    pub _downsample: i32,
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CommandBufferBlurryScreenGrab => ""
    ."CommandBufferBlurryScreenGrab"
);
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl std::ops::Deref for CommandBufferBlurryScreenGrab {
    type Target = CommandBufferGOCore;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl std::ops::DerefMut for CommandBufferBlurryScreenGrab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl CommandBufferBlurryScreenGrab {
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
    pub fn CreateCommandBuffer(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Rendering::CommandBuffer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Rendering::CommandBuffer = __cordl_object
            .invoke("CreateCommandBuffer", (camera))?;
        Ok(__cordl_ret)
    }
    pub fn CamerasDict(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Camera,
            *mut CommandBufferOwners,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::Camera,
            *mut CommandBufferOwners,
        > = __cordl_object.invoke("CamerasDict", ())?;
        Ok(__cordl_ret)
    }
    pub fn CommandBufferCameraEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CameraEvent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::CameraEvent = __cordl_object
            .invoke("CommandBufferCameraEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl quest_hook::libil2cpp::ObjectType for CommandBufferBlurryScreenGrab {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "CommandBufferBlurryScreenGrab")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBufferBlurryScreenGrab {
    __cordl_parent: crate::GlobalNamespace::CommandBufferGOCore,
    pub _kawaseBlurRenderer: *mut crate::GlobalNamespace::KawaseBlurRendererSO,
    pub _kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _cameraEvent: crate::UnityEngine::Rendering::CameraEvent,
    pub _downsample: i32,
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CommandBufferBlurryScreenGrab
    => ""."CommandBufferBlurryScreenGrab"
);
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl std::ops::Deref for crate::GlobalNamespace::CommandBufferBlurryScreenGrab {
    type Target = crate::GlobalNamespace::CommandBufferGOCore;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl std::ops::DerefMut for crate::GlobalNamespace::CommandBufferBlurryScreenGrab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl crate::GlobalNamespace::CommandBufferBlurryScreenGrab {
    pub fn CamerasDict(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::UnityEngine::Camera,
                *mut crate::GlobalNamespace::CommandBufferOwners,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::UnityEngine::Camera,
                *mut crate::GlobalNamespace::CommandBufferOwners,
            >,
        > = __cordl_object.invoke("CamerasDict", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CommandBufferCameraEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CameraEvent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::CameraEvent = __cordl_object
            .invoke("CommandBufferCameraEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCommandBuffer(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        > = __cordl_object.invoke("CreateCommandBuffer", (camera))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "CommandBufferBlurryScreenGrab")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CommandBufferBlurryScreenGrab {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

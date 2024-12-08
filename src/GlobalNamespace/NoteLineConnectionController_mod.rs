#[cfg(feature = "NoteLineConnectionController")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteLineConnectionController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lineRenderer: *mut crate::UnityEngine::LineRenderer,
    pub _playerTransforms: *mut PlayerTransforms,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _colorManager: *mut ColorManager,
    pub didFinishEvent: *mut crate::System::Action_1<*mut NoteLineConnectionController>,
    pub _noteController0: *mut NoteController,
    pub _noteController1: *mut NoteController,
    pub _color0: crate::UnityEngine::Color,
    pub _color1: crate::UnityEngine::Color,
    pub _fadeOutStartDistance: f32,
    pub _fadeOutEndDistance: f32,
    pub _noteTime: f32,
    pub _didFinish: bool,
}
#[cfg(feature = "NoteLineConnectionController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteLineConnectionController => ""
    ."NoteLineConnectionController"
);
#[cfg(feature = "NoteLineConnectionController")]
impl std::ops::Deref for NoteLineConnectionController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLineConnectionController")]
impl std::ops::DerefMut for NoteLineConnectionController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLineConnectionController")]
impl NoteLineConnectionController {
    #[cfg(feature = "NoteLineConnectionController+Pool")]
    pub type Pool = crate::GlobalNamespace::NoteLineConnectionController_Pool;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Setup(
        &mut self,
        noteController0: *mut NoteController,
        noteController1: *mut NoteController,
        fadeOutStartDistance: f32,
        fadeOutEndDistance: f32,
        noteTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Setup",
                (
                    noteController0,
                    noteController1,
                    fadeOutStartDistance,
                    fadeOutEndDistance,
                    noteTime,
                ),
            )?;
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
    pub fn UpdatePositionsAndColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePositionsAndColors", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteLineConnectionController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut NoteLineConnectionController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteLineConnectionController")]
impl quest_hook::libil2cpp::ObjectType for NoteLineConnectionController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteLineConnectionController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteLineConnectionController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut NoteLineConnectionController>,
}
#[cfg(feature = "NoteLineConnectionController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteLineConnectionController_Pool => ""
    ."NoteLineConnectionController/Pool"
);
#[cfg(feature = "NoteLineConnectionController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::NoteLineConnectionController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut NoteLineConnectionController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLineConnectionController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteLineConnectionController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteLineConnectionController+Pool")]
impl crate::GlobalNamespace::NoteLineConnectionController_Pool {
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
#[cfg(feature = "NoteLineConnectionController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteLineConnectionController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

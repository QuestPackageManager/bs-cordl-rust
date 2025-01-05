#[cfg(feature = "NoteJumpStartScaleUp")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteJumpStartScaleUp {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _fullScaleJumpPart: f32,
    pub _targetTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _noteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteController,
    >,
    pub _noteJump: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteJump>,
}
#[cfg(feature = "NoteJumpStartScaleUp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteJumpStartScaleUp => ""
    ."NoteJumpStartScaleUp"
);
#[cfg(feature = "NoteJumpStartScaleUp")]
impl std::ops::Deref for crate::GlobalNamespace::NoteJumpStartScaleUp {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpStartScaleUp")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteJumpStartScaleUp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpStartScaleUp")]
impl crate::GlobalNamespace::NoteJumpStartScaleUp {
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
    pub fn HandleNoteControllerDidInit(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteControllerDidInit", (noteController))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteJumpDidUpdateProgress(
        &mut self,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteJumpDidUpdateProgress", (progress))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScale(
        &mut self,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScale", (progress))?;
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
}
#[cfg(feature = "NoteJumpStartScaleUp")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteJumpStartScaleUp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoteJumpStartScaleUp")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INoteControllerDidInitEvent>,
> for crate::GlobalNamespace::NoteJumpStartScaleUp {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INoteControllerDidInitEvent,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoteJumpStartScaleUp")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INoteControllerDidInitEvent>,
> for crate::GlobalNamespace::NoteJumpStartScaleUp {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INoteControllerDidInitEvent,
    > {
        unsafe { std::mem::transmute(self) }
    }
}

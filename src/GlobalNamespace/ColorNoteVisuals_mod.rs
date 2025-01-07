#[cfg(feature = "ColorNoteVisuals")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorNoteVisuals {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultColorAlpha: f32,
    pub _noteController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteControllerBase,
    >,
    pub _materialPropertyBlockControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MaterialPropertyBlockController,
            >,
        >,
    >,
    pub _arrowMeshRenderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
        >,
    >,
    pub _circleMeshRenderers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
        >,
    >,
    pub _colorManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorManager>,
    pub didInitEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorNoteVisuals>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteControllerBase>,
        >,
    >,
    pub _noteColor: crate::UnityEngine::Color,
}
#[cfg(feature = "ColorNoteVisuals")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ColorNoteVisuals {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ColorNoteVisuals";
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
#[cfg(feature = "ColorNoteVisuals")]
impl std::ops::Deref for crate::GlobalNamespace::ColorNoteVisuals {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorNoteVisuals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl crate::GlobalNamespace::ColorNoteVisuals {
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
    pub fn HandleNoteControllerNoteDidPassJumpThreeQuarters(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteControllerNoteDidPassJumpThreeQuarters",
                (noteController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteControllerNoteDidStartDissolving(
        &mut self,
        noteController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteControllerBase,
        >,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteControllerNoteDidStartDissolving",
                (noteController, duration),
            )?;
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
    pub fn add_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorNoteVisuals>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteControllerBase>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didInitEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didInitEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorNoteVisuals>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteControllerBase>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didInitEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showArrow(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showArrow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showCircle(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showCircle", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsRef<crate::GlobalNamespace::INoteControllerDidInitEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteControllerDidInitEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsMut<crate::GlobalNamespace::INoteControllerDidInitEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteControllerDidInitEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsRef<crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ColorNoteVisuals")]
impl AsMut<crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent>
for crate::GlobalNamespace::ColorNoteVisuals {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent {
        unsafe { std::mem::transmute(self) }
    }
}

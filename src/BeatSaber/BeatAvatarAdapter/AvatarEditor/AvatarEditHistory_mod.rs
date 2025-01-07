#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarEditHistory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub disableNextSnapshotOverride: bool,
    pub _snapShots: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot,
        >,
    >,
    pub _currentDataId: i32,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarAdapter.AvatarEditor";
    const CLASS_NAME: &'static str = "AvatarEditHistory";
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
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Redo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Redo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Undo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Undo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateEditHistory(
        &mut self,
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarEditPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateEditHistory", (avatarData, avatarEditPart))?;
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
    pub fn get_currentSnapShot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarHistorySnapshot = __cordl_object
            .invoke("get_currentSnapShot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastEditedPart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::BeatAvatarSDK::AvatarPart> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::BeatAvatarSDK::AvatarPart = __cordl_object
            .invoke("get_lastEditedPart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_redoAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_redoAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_undoAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_undoAvailable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+AvatarEditHistory")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarEditHistory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "MirroredNoteController_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MirroredNoteController_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: NoteControllerBase,
    pub _noteTransform: *mut crate::UnityEngine::Transform,
    pub followedNote: T,
    pub _followedNoteTransform: *mut crate::UnityEngine::Transform,
    pub _didInitEvent: *mut LazyCopyHashSet_1<*mut INoteControllerDidInitEvent>,
    pub _noteDidPassJumpThreeQuartersEvent: *mut LazyCopyHashSet_1<
        *mut INoteControllerNoteDidPassJumpThreeQuartersEvent,
    >,
    pub _noteDidStartDissolvingEvent: *mut LazyCopyHashSet_1<
        *mut INoteControllerNoteDidStartDissolvingEvent,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "MirroredNoteController_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MirroredNoteController_1 < T > => ""
    ."MirroredNoteController`1" < T >
);
#[cfg(feature = "MirroredNoteController_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref for MirroredNoteController_1<T> {
    type Target = NoteControllerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredNoteController_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut for MirroredNoteController_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirroredNoteController_1")]
impl<T: quest_hook::libil2cpp::Type> MirroredNoteController_1<T> {
    pub fn get_noteDidStartDissolvingEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut INoteControllerNoteDidStartDissolvingEvent>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<
            *mut INoteControllerNoteDidStartDissolvingEvent,
        > = __cordl_object.invoke("get_noteDidStartDissolvingEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidPassJumpThreeQuarters(
        &mut self,
        noteController: *mut NoteControllerBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteControllerNoteDidPassJumpThreeQuarters",
                (noteController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_noteData(&mut self) -> quest_hook::libil2cpp::Result<*mut NoteData>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut NoteData = __cordl_object.invoke("get_noteData", ())?;
        Ok(__cordl_ret)
    }
    pub fn Mirror(
        &mut self,
        noteController: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (noteController))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_didInitEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut INoteControllerDidInitEvent>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<*mut INoteControllerDidInitEvent> = __cordl_object
            .invoke("get_didInitEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePositionAndRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePositionAndRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveListeners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveListeners", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_noteDidPassJumpThreeQuartersEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut INoteControllerNoteDidPassJumpThreeQuartersEvent>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<
            *mut INoteControllerNoteDidPassJumpThreeQuartersEvent,
        > = __cordl_object.invoke("get_noteDidPassJumpThreeQuartersEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteControllerNoteDidStartDissolving(
        &mut self,
        noteController: *mut NoteControllerBase,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteControllerNoteDidStartDissolving",
                (noteController, duration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
        hide: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", (hide))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
#[cfg(feature = "MirroredNoteController_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for MirroredNoteController_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "NoteControllerBase")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteControllerBase {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "NoteControllerBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteControllerBase => ""
    ."NoteControllerBase"
);
#[cfg(feature = "NoteControllerBase")]
impl std::ops::Deref for crate::GlobalNamespace::NoteControllerBase {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteControllerBase")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteControllerBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteControllerBase")]
impl crate::GlobalNamespace::NoteControllerBase {
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
    pub fn get_didInitEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerDidInitEvent,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerDidInitEvent,
            >,
        > = __cordl_object.invoke("get_didInitEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteData> = __cordl_object
            .invoke("get_noteData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteDidPassJumpThreeQuartersEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerNoteDidPassJumpThreeQuartersEvent,
            >,
        > = __cordl_object.invoke("get_noteDidPassJumpThreeQuartersEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteDidStartDissolvingEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                *mut crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent,
            >,
        > = __cordl_object.invoke("get_noteDidStartDissolvingEvent", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoteControllerBase")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteControllerBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

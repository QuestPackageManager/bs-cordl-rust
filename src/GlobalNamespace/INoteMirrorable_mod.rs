#[cfg(feature = "INoteMirrorable")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteMirrorable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteMirrorable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INoteMirrorable => ""
    ."INoteMirrorable"
);
#[cfg(feature = "INoteMirrorable")]
impl std::ops::Deref for crate::GlobalNamespace::INoteMirrorable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteMirrorable")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteMirrorable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteMirrorable")]
impl crate::GlobalNamespace::INoteMirrorable {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
    pub fn get_noteDidStartDissolvingEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::INoteControllerNoteDidStartDissolvingEvent,
                >,
            >,
        > = __cordl_object.invoke("get_noteDidStartDissolvingEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_noteTransform", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "INoteMirrorable")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::INoteMirrorable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

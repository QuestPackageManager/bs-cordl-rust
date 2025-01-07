#[cfg(feature = "IGameNoteMirrorable")]
#[repr(C)]
#[derive(Debug)]
pub struct IGameNoteMirrorable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IGameNoteMirrorable")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::IGameNoteMirrorable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IGameNoteMirrorable";
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
#[cfg(feature = "IGameNoteMirrorable")]
impl std::ops::Deref for crate::GlobalNamespace::IGameNoteMirrorable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IGameNoteMirrorable")]
impl std::ops::DerefMut for crate::GlobalNamespace::IGameNoteMirrorable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IGameNoteMirrorable")]
impl crate::GlobalNamespace::IGameNoteMirrorable {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_gameplayType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteData_GameplayType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteData_GameplayType = __cordl_object
            .invoke("get_gameplayType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NoteMovement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMovement,
        > = __cordl_object.invoke("get_noteMovement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteVisualModifierType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteVisualModifierType = __cordl_object
            .invoke("get_noteVisualModifierType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IGameNoteMirrorable")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IGameNoteMirrorable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IGameNoteMirrorable")]
impl AsRef<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::IGameNoteMirrorable {
    fn as_ref(&self) -> &crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IGameNoteMirrorable")]
impl AsMut<crate::GlobalNamespace::INoteMirrorable>
for crate::GlobalNamespace::IGameNoteMirrorable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::INoteMirrorable {
        unsafe { std::mem::transmute(self) }
    }
}

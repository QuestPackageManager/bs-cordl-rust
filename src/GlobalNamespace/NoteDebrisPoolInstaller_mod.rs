#[cfg(feature = "NoteDebrisPoolInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisPoolInstaller {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::Zenject::ScriptableObjectInstaller>,
    pub _normalNoteDebrisHDPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _normalNoteDebrisLWPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _burstSliderHeadNoteDebrisHDPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _burstSliderHeadNoteDebrisLWPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _burstSliderElementNoteHDPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _burstSliderElementNoteLWPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteDebris,
    >,
    pub _noteDebrisHDConditionVariable: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BoolSO,
    >,
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteDebrisPoolInstaller => ""
    ."NoteDebrisPoolInstaller"
);
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisPoolInstaller {
    type Target = quest_hook::libil2cpp::Gc<crate::Zenject::ScriptableObjectInstaller>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteDebrisPoolInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl crate::GlobalNamespace::NoteDebrisPoolInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
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
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteDebrisPoolInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "NoteDebrisPoolInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisPoolInstaller {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
    pub _normalNoteDebrisHDPrefab: *mut NoteDebris,
    pub _normalNoteDebrisLWPrefab: *mut NoteDebris,
    pub _burstSliderHeadNoteDebrisHDPrefab: *mut NoteDebris,
    pub _burstSliderHeadNoteDebrisLWPrefab: *mut NoteDebris,
    pub _burstSliderElementNoteHDPrefab: *mut NoteDebris,
    pub _burstSliderElementNoteLWPrefab: *mut NoteDebris,
    pub _noteDebrisHDConditionVariable: *mut BoolSO,
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NoteDebrisPoolInstaller => ""."NoteDebrisPoolInstaller"
);
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl std::ops::Deref for NoteDebrisPoolInstaller {
    type Target = crate::Zenject::ScriptableObjectInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl std::ops::DerefMut for NoteDebrisPoolInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl NoteDebrisPoolInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl quest_hook::libil2cpp::ObjectType for NoteDebrisPoolInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

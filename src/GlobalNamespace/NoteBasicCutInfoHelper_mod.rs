#[cfg(feature = "NoteBasicCutInfoHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteBasicCutInfoHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteBasicCutInfoHelper => ""
    ."NoteBasicCutInfoHelper"
);
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::Deref for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl crate::GlobalNamespace::NoteBasicCutInfoHelper {
    pub const kMinBladeSpeedForCut: f32 = 2f32;
}
#[cfg(feature = "NoteBasicCutInfoHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteBasicCutInfoHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

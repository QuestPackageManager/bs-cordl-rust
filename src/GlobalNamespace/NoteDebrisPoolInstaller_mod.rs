#[cfg(feature = "NoteDebrisPoolInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDebrisPoolInstaller {
    __cordl_parent: crate::Zenject::ScriptableObjectInstaller,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoteDebrisPoolInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteDebrisPoolInstaller";
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
#[cfg(feature = "NoteDebrisPoolInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::NoteDebrisPoolInstaller {
    type Target = crate::Zenject::ScriptableObjectInstaller;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InstallBindings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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

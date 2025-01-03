#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteJumpDurationTypeSettingsDropdown {
    __cordl_parent: crate::GlobalNamespace::ValueDropdownController_1<
        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    >,
}
#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown => ""
    ."NoteJumpDurationTypeSettingsDropdown"
);
#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
impl std::ops::Deref for crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown {
    type Target = crate::GlobalNamespace::ValueDropdownController_1<
        crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
impl crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown {
    pub fn GetNamedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Tuple_2<
                    crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::Tuple_2<
                    crate::GlobalNamespace::NoteJumpDurationTypeSettings,
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = __cordl_object.invoke("GetNamedValues", ())?;
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
#[cfg(feature = "NoteJumpDurationTypeSettingsDropdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

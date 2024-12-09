#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteJumpStartBeatOffsetDropdown {
    __cordl_parent: crate::GlobalNamespace::ValueDropdownController_1<f32>,
}
#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteJumpStartBeatOffsetDropdown
    => ""."NoteJumpStartBeatOffsetDropdown"
);
#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
impl std::ops::Deref for crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown {
    type Target = crate::GlobalNamespace::ValueDropdownController_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
impl crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown {
    pub fn GetNamedValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<f32, *mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::Tuple_2<f32, *mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetNamedValues", ())?;
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
#[cfg(feature = "NoteJumpStartBeatOffsetDropdown")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "MidiParser+MidiTrack")]
#[repr(C)]
#[derive(Debug)]
pub struct MidiTrack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Index: i32,
    pub MidiEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::MidiParser::MidiEvent>,
    >,
}
#[cfg(feature = "MidiParser+MidiTrack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MidiParser::MidiTrack => "MidiParser"
    ."MidiTrack"
);
#[cfg(feature = "MidiParser+MidiTrack")]
impl std::ops::Deref for crate::MidiParser::MidiTrack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiTrack")]
impl std::ops::DerefMut for crate::MidiParser::MidiTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiTrack")]
impl crate::MidiParser::MidiTrack {
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
#[cfg(feature = "MidiParser+MidiTrack")]
impl quest_hook::libil2cpp::ObjectType for crate::MidiParser::MidiTrack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

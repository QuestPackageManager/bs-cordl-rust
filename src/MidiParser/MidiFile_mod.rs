#[cfg(feature = "MidiParser+MidiFile")]
#[repr(C)]
#[derive(Debug)]
pub struct MidiFile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub format: i32,
    pub ticksPerQuarterNote: i32,
    pub tracks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::MidiParser::MidiTrack,
    >,
    pub tracksCount: i32,
}
#[cfg(feature = "MidiParser+MidiFile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MidiParser::MidiFile => "MidiParser"."MidiFile"
);
#[cfg(feature = "MidiParser+MidiFile")]
impl std::ops::Deref for crate::MidiParser::MidiFile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiFile")]
impl std::ops::DerefMut for crate::MidiParser::MidiFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiFile")]
impl crate::MidiParser::MidiFile {
    #[cfg(feature = "MidiParser+MidiFile+Reader")]
    pub type Reader = crate::MidiParser::MidiFile_Reader;
    pub fn New_Il2CppArray1(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseMetaEvent(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
        metaEventType: u8,
        data1: quest_hook::libil2cpp::ByRefMut<i32>,
        data2: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMetaEvent", (data, position, metaEventType, data1, data2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTrack(
        index: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        position: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MidiParser::MidiTrack>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::MidiParser::MidiTrack> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseTrack", (index, data, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MidiParser+MidiFile")]
impl quest_hook::libil2cpp::ObjectType for crate::MidiParser::MidiFile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MidiParser+MidiFile+Reader")]
#[repr(C)]
#[derive(Debug)]
pub struct MidiFile_Reader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MidiParser+MidiFile+Reader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::MidiParser::MidiFile_Reader => "MidiParser"
    ."MidiFile/Reader"
);
#[cfg(feature = "MidiParser+MidiFile+Reader")]
impl std::ops::Deref for crate::MidiParser::MidiFile_Reader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiFile+Reader")]
impl std::ops::DerefMut for crate::MidiParser::MidiFile_Reader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MidiParser+MidiFile+Reader")]
impl crate::MidiParser::MidiFile_Reader {
    pub fn Read16(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read16", (data, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read32(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read32", (data, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read8(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read8", (data, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllBytesFromStream(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllBytesFromStream", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadString(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadString", (data, i, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadVarInt(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadVarInt", (data, i))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MidiParser+MidiFile+Reader")]
impl quest_hook::libil2cpp::ObjectType for crate::MidiParser::MidiFile_Reader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

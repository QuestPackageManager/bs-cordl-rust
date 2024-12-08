#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoChunkParser_Chunk {
    __cordl_parent: crate::System::Object,
    pub Bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Offset: i32,
}
#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::MonoChunkParser_Chunk =>
    "System.Net"."MonoChunkParser/Chunk"
);
#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
impl std::ops::Deref for crate::System::Net::MonoChunkParser_Chunk {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
impl std::ops::DerefMut for crate::System::Net::MonoChunkParser_Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
impl crate::System::Net::MonoChunkParser_Chunk {
    pub fn New(
        chunk: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (chunk))?;
        Ok(__cordl_object)
    }
    pub fn Read(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Read", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        chunk: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (chunk))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::MonoChunkParser_Chunk {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+MonoChunkParser")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoChunkParser {
    __cordl_parent: crate::System::Object,
    pub headers: *mut crate::System::Net::WebHeaderCollection,
    pub chunkSize: i32,
    pub chunkRead: i32,
    pub totalWritten: i32,
    pub state: crate::System::Net::MonoChunkParser_State,
    pub saved: *mut crate::System::Text::StringBuilder,
    pub sawCR: bool,
    pub gotit: bool,
    pub trailerState: i32,
    pub chunks: *mut crate::System::Collections::ArrayList,
}
#[cfg(feature = "System+Net+MonoChunkParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::MonoChunkParser => "System.Net"
    ."MonoChunkParser"
);
#[cfg(feature = "System+Net+MonoChunkParser")]
impl std::ops::Deref for crate::System::Net::MonoChunkParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkParser")]
impl std::ops::DerefMut for crate::System::Net::MonoChunkParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+MonoChunkParser")]
impl crate::System::Net::MonoChunkParser {
    #[cfg(feature = "System+Net+MonoChunkParser+State")]
    pub type State = crate::System::Net::MonoChunkParser_State;
    #[cfg(feature = "System+Net+MonoChunkParser+Chunk")]
    pub type Chunk = crate::System::Net::MonoChunkParser_Chunk;
    pub fn GetChunkSize(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::MonoChunkParser_State> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::MonoChunkParser_State = __cordl_object
            .invoke("GetChunkSize", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWrite(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalWrite", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        headers: *mut crate::System::Net::WebHeaderCollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (headers))?;
        Ok(__cordl_object)
    }
    pub fn Read(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Read", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ReadBody(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::MonoChunkParser_State> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::MonoChunkParser_State = __cordl_object
            .invoke("ReadBody", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ReadCRLF(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::MonoChunkParser_State> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::MonoChunkParser_State = __cordl_object
            .invoke("ReadCRLF", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ReadFromChunks(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadFromChunks", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn ReadTrailer(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: quest_hook::libil2cpp::ByRefMut<i32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::MonoChunkParser_State> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::MonoChunkParser_State = __cordl_object
            .invoke("ReadTrailer", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        headers: *mut crate::System::Net::WebHeaderCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (headers))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChunkLeft(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ChunkLeft", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DataAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DataAvailable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WantMore(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_WantMore", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+MonoChunkParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::MonoChunkParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+MonoChunkParser+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonoChunkParser_State {
    Body = 2i32,
    BodyFinished = 3i32,
    None = 0i32,
    PartialSize = 1i32,
    Trailer = 4i32,
}
#[cfg(feature = "System+Net+MonoChunkParser+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::MonoChunkParser_State =>
    "System.Net"."MonoChunkParser/State"
);

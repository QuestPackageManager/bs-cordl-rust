#[cfg(feature = "System+Net+ChunkedInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct ChunkedInputStream {
    __cordl_parent: crate::System::Net::RequestStream,
    pub disposed: bool,
    pub decoder: quest_hook::libil2cpp::Gc<crate::System::Net::MonoChunkParser>,
    pub context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    pub no_more_data: bool,
}
#[cfg(feature = "System+Net+ChunkedInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ChunkedInputStream => "System.Net"
    ."ChunkedInputStream"
);
#[cfg(feature = "System+Net+ChunkedInputStream")]
impl std::ops::Deref for crate::System::Net::ChunkedInputStream {
    type Target = crate::System::Net::RequestStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream")]
impl std::ops::DerefMut for crate::System::Net::ChunkedInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream")]
impl crate::System::Net::ChunkedInputStream {
    #[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
    pub type ReadBufferState = crate::System::Net::ChunkedInputStream_ReadBufferState;
    pub fn BeginRead(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        cback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginRead", (buffer, offset, count, cback, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndRead(
        &mut self,
        ares: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndRead", (ares))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, stream, buffer, offset, length))?;
        Ok(__cordl_object.into())
    }
    pub fn OnRead(
        &mut self,
        base_ares: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRead", (base_ares))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        &mut self,
        buffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, stream, buffer, offset, length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ChunkedInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
#[repr(C)]
#[derive(Debug)]
pub struct ChunkedInputStream_ReadBufferState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub Offset: i32,
    pub Count: i32,
    pub InitialCount: i32,
    pub Ares: quest_hook::libil2cpp::Gc<crate::System::Net::HttpStreamAsyncResult>,
}
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ChunkedInputStream_ReadBufferState
    => "System.Net"."ChunkedInputStream/ReadBufferState"
);
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
impl std::ops::Deref for crate::System::Net::ChunkedInputStream_ReadBufferState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
impl std::ops::DerefMut for crate::System::Net::ChunkedInputStream_ReadBufferState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
impl crate::System::Net::ChunkedInputStream_ReadBufferState {
    pub fn New(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        ares: quest_hook::libil2cpp::Gc<crate::System::Net::HttpStreamAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (buffer, offset, count, ares))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        ares: quest_hook::libil2cpp::Gc<crate::System::Net::HttpStreamAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (buffer, offset, count, ares))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ChunkedInputStream+ReadBufferState")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::ChunkedInputStream_ReadBufferState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+IO+StreamWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct StreamWriter {
    __cordl_parent: crate::System::IO::TextWriter,
    pub _stream: *mut crate::System::IO::Stream,
    pub _encoding: *mut crate::System::Text::Encoding,
    pub _encoder: *mut crate::System::Text::Encoder,
    pub _byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _charBuffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _charPos: i32,
    pub _charLen: i32,
    pub _autoFlush: bool,
    pub _haveWrittenPreamble: bool,
    pub _closable: bool,
    pub _asyncWriteTask: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(feature = "System+IO+StreamWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::StreamWriter => "System.IO"
    ."StreamWriter"
);
#[cfg(feature = "System+IO+StreamWriter")]
impl std::ops::Deref for crate::System::IO::StreamWriter {
    type Target = crate::System::IO::TextWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamWriter")]
impl std::ops::DerefMut for crate::System::IO::StreamWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+StreamWriter")]
impl crate::System::IO::StreamWriter {
    #[cfg(feature = "System+IO+StreamWriter+_DisposeAsyncCore_d__33")]
    pub type _DisposeAsyncCore_d__33 = crate::System::IO::StreamWriter__DisposeAsyncCore_d__33;
    #[cfg(feature = "System+IO+StreamWriter+_FlushAsyncInternal_d__74")]
    pub type _FlushAsyncInternal_d__74 = crate::System::IO::StreamWriter__FlushAsyncInternal_d__74;
    #[cfg(feature = "System+IO+StreamWriter+_WriteAsyncInternal_d__57")]
    pub type _WriteAsyncInternal_d__57 = crate::System::IO::StreamWriter__WriteAsyncInternal_d__57;
    #[cfg(feature = "System+IO+StreamWriter+_WriteAsyncInternal_d__59")]
    pub type _WriteAsyncInternal_d__59 = crate::System::IO::StreamWriter__WriteAsyncInternal_d__59;
    #[cfg(feature = "System+IO+StreamWriter+_WriteAsyncInternal_d__62")]
    pub type _WriteAsyncInternal_d__62 = crate::System::IO::StreamWriter__WriteAsyncInternal_d__62;
    pub fn CheckAsyncTaskInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAsyncTaskInProgress", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
        Ok(__cordl_ret)
    }
    pub fn CloseStreamFromDispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseStreamFromDispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisposeAsyncCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsyncCore", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FlushAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn FlushAsyncInternal(
        &mut self,
        flushStream: bool,
        flushEncoder: bool,
        sCharBuffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        sCharPos: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "FlushAsyncInternal",
                (flushStream, flushEncoder, sCharBuffer, sCharPos, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Flush_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn Flush__cordl_bool__cordl_bool1(
        &mut self,
        flushStream: bool,
        flushEncoder: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", (flushStream, flushEncoder))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        streamArg: *mut crate::System::IO::Stream,
        encodingArg: *mut crate::System::Text::Encoding,
        bufferSize: i32,
        shouldLeaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (streamArg, encodingArg, bufferSize, shouldLeaveOpen))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Stream1(
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Encoding2(
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Encoding_i32_3(
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding, bufferSize))?;
        Ok(__cordl_object)
    }
    pub fn New_Stream_Encoding_i32__cordl_bool4(
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
        leaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding, bufferSize, leaveOpen))?;
        Ok(__cordl_object)
    }
    pub fn New_String5(
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_bool6(
        path: *mut crate::System::String,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, append))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_bool_Encoding_i32_7(
        path: *mut crate::System::String,
        append: bool,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, append, encoding, bufferSize))?;
        Ok(__cordl_object)
    }
    pub fn WriteAsync_Il2CppArray_i32_i32_2(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteAsync", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAsync_String1(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteAsync", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAsync__cordl_char0(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteAsync", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteLine(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteLine", (value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSpan(
        &mut self,
        buffer: crate::System::ReadOnlySpan_1<char>,
        appendNewLine: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSpan", (buffer, appendNewLine))?;
        Ok(__cordl_ret)
    }
    pub fn Write_Il2CppArray1(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn Write_Il2CppArray_i32_i32_2(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn Write_String3(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Write__cordl_char0(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream1(
        &mut self,
        stream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Encoding2(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Encoding_i32_3(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Stream_Encoding_i32__cordl_bool4(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
        leaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding, bufferSize, leaveOpen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String5(
        &mut self,
        path: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_bool6(
        &mut self,
        path: *mut crate::System::String,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, append))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_bool_Encoding_i32_7(
        &mut self,
        path: *mut crate::System::String,
        append: bool,
        encoding: *mut crate::System::Text::Encoding,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, append, encoding, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("get_BaseStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LeaveOpen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_LeaveOpen", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_AutoFlush(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AutoFlush", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CharPos_Prop(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CharPos_Prop", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_HaveWrittenPreamble_Prop(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_HaveWrittenPreamble_Prop", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+IO+StreamWriter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::StreamWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

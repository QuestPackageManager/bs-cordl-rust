#[cfg(feature = "System+IO+StreamWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct StreamWriter {
    __cordl_parent: crate::System::IO::TextWriter,
    pub _stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub _encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    pub _encoder: quest_hook::libil2cpp::Gc<crate::System::Text::Encoder>,
    pub _byteBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _charPos: i32,
    pub _charLen: i32,
    pub _autoFlush: bool,
    pub _haveWrittenPreamble: bool,
    pub _closable: bool,
    pub _asyncWriteTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task,
    >,
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
    pub fn CheckAsyncTaskInProgress(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckAsyncTaskInProgress", ())?;
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
    pub fn CloseStreamFromDispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseStreamFromDispose", (disposing))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsyncCore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = __cordl_object
            .invoke("DisposeAsyncCore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FlushAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsyncInternal_StreamWriter__cordl_bool_Il2CppArray_i32__cordl_bool_Encoding_Encoder_Il2CppArray_Stream_CancellationToken1(
        _this: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
        flushStream: bool,
        flushEncoder: bool,
        charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charPos: i32,
        haveWrittenPreamble: bool,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        encoder: quest_hook::libil2cpp::Gc<crate::System::Text::Encoder>,
        byteBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FlushAsyncInternal",
                (
                    _this,
                    flushStream,
                    flushEncoder,
                    charBuffer,
                    charPos,
                    haveWrittenPreamble,
                    encoding,
                    encoder,
                    byteBuffer,
                    stream,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsyncInternal__cordl_bool_Il2CppArray_i32_CancellationToken0(
        &mut self,
        flushStream: bool,
        flushEncoder: bool,
        sCharBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        sCharPos: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke(
                "FlushAsyncInternal",
                (flushStream, flushEncoder, sCharBuffer, sCharPos, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        streamArg: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encodingArg: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
        shouldLeaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (streamArg, encodingArg, bufferSize, shouldLeaveOpen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString5(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool6(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, append))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool_Encoding7(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, append, encoding))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool_Encoding_i32_8(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, append, encoding, bufferSize))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream1(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_Encoding2(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_Encoding_i32_3(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding, bufferSize))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Stream_Encoding_i32__cordl_bool4(
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
        leaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream, encoding, bufferSize, leaveOpen))?;
        Ok(__cordl_object.into())
    }
    pub fn ThrowAsyncIOInProgress() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAsyncIOInProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsyncInternal_Il2CppString1(
        _this: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charPos: i32,
        charLen: i32,
        coreNewLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        autoFlush: bool,
        appendNewLine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteAsyncInternal",
                (
                    _this,
                    value,
                    charBuffer,
                    charPos,
                    charLen,
                    coreNewLine,
                    autoFlush,
                    appendNewLine,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsyncInternal_ReadOnlyMemory_1_CancellationToken2(
        _this: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
        source: crate::System::ReadOnlyMemory_1<char>,
        charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charPos: i32,
        charLen: i32,
        coreNewLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        autoFlush: bool,
        appendNewLine: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteAsyncInternal",
                (
                    _this,
                    source,
                    charBuffer,
                    charPos,
                    charLen,
                    coreNewLine,
                    autoFlush,
                    appendNewLine,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsyncInternal__cordl_char0(
        _this: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
        value: char,
        charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charPos: i32,
        charLen: i32,
        coreNewLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        autoFlush: bool,
        appendNewLine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteAsyncInternal",
                (
                    _this,
                    value,
                    charBuffer,
                    charPos,
                    charLen,
                    coreNewLine,
                    autoFlush,
                    appendNewLine,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppArray_i32_i32_2(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteAsync", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppString1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteAsync", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync__cordl_char0(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("WriteAsync", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLine(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteLine", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppArray1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppArray_i32_i32_2(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppString3(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString5(
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
    pub fn _ctor_Il2CppString__cordl_bool6(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, append))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool_Encoding7(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, append, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool_Encoding_i32_8(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        append: bool,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, append, encoding, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream1(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_Encoding2(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_Encoding_i32_3(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Stream_Encoding_i32__cordl_bool4(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        bufferSize: i32,
        leaveOpen: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream, encoding, bufferSize, leaveOpen))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("get_BaseStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LeaveOpen(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_LeaveOpen", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UTF8NoBOM() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UTF8NoBOM", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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

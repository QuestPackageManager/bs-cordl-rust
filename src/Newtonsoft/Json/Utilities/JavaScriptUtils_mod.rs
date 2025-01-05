#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct JavaScriptUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::JavaScriptUtils =>
    "Newtonsoft.Json.Utilities"."JavaScriptUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    pub const EscapedUnicodeText: &'static str = "!";
    pub const UnicodeTextLength: i32 = 6i32;
    pub fn FirstCharToEscape(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstCharToEscape", (s, charEscapeFlags, stringEscapeHandling))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharEscapeFlags(
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCharEscapeFlags", (stringEscapeHandling, quoteChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldEscapeJavaScriptString(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldEscapeJavaScriptString", (s, charEscapeFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToEscapedJavaScriptString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        appendDelimiters: bool,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ToEscapedJavaScriptString",
                (value, delimiter, appendDelimiters, stringEscapeHandling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateConstructorValue(
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        integer: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i64>>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDateConstructorValue", (reader, integer, errorMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateFromConstructorJson(
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        dateTime: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDateFromConstructorJson", (reader, dateTime, errorMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCharAsync(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        c: char,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteCharAsync", (task, writer, c, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lastWritePosition: i32,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        client: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
        writeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync",
                (
                    writer,
                    s,
                    lastWritePosition,
                    charEscapeFlags,
                    stringEscapeHandling,
                    client,
                    writeBuffer,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEscapedJavaScriptString(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        appendDelimiters: bool,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
        writeBuffer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEscapedJavaScriptString",
                (
                    writer,
                    s,
                    delimiter,
                    appendDelimiters,
                    charEscapeFlags,
                    stringEscapeHandling,
                    bufferPool,
                    writeBuffer,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEscapedJavaScriptStringAsync(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        appendDelimiters: bool,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        client: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
        writeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEscapedJavaScriptStringAsync",
                (
                    writer,
                    s,
                    delimiter,
                    appendDelimiters,
                    charEscapeFlags,
                    stringEscapeHandling,
                    client,
                    writeBuffer,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEscapedJavaScriptStringWithDelimitersAsync_Task_TextWriter_Il2CppString__cordl_char_Il2CppArray_StringEscapeHandling_JsonTextWriter_Il2CppArray_CancellationToken1(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        client: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
        writeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEscapedJavaScriptStringWithDelimitersAsync",
                (
                    task,
                    writer,
                    s,
                    delimiter,
                    charEscapeFlags,
                    stringEscapeHandling,
                    client,
                    writeBuffer,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEscapedJavaScriptStringWithDelimitersAsync_TextWriter_Il2CppString__cordl_char_Il2CppArray_StringEscapeHandling_JsonTextWriter_Il2CppArray_CancellationToken0(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        delimiter: char,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        client: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
        writeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEscapedJavaScriptStringWithDelimitersAsync",
                (
                    writer,
                    s,
                    delimiter,
                    charEscapeFlags,
                    stringEscapeHandling,
                    client,
                    writeBuffer,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteEscapedJavaScriptStringWithoutDelimitersAsync(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        client: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
        writeBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteEscapedJavaScriptStringWithoutDelimitersAsync",
                (
                    writer,
                    s,
                    charEscapeFlags,
                    stringEscapeHandling,
                    client,
                    writeBuffer,
                    cancellationToken,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

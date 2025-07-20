#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct JavaScriptUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "JavaScriptUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                        ),
                        i32,
                        3usize,
                    >("FirstCharToEscape")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FirstCharToEscape", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (s, charEscapeFlags, stringEscapeHandling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharEscapeFlags(
        stringEscapeHandling: crate::Newtonsoft::Json::StringEscapeHandling,
        quoteChar: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Newtonsoft::Json::StringEscapeHandling, char),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<bool>,
                        >,
                        2usize,
                    >("GetCharEscapeFlags")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCharEscapeFlags", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        > = unsafe { method.invoke_unchecked((), (stringEscapeHandling, quoteChar))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShouldEscapeJavaScriptString(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        charEscapeFlags: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ShouldEscapeJavaScriptString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ShouldEscapeJavaScriptString", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, charEscapeFlags))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            bool,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("ToEscapedJavaScriptString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToEscapedJavaScriptString", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (value, delimiter, appendDelimiters, stringEscapeHandling),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateConstructorValue(
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        integer: quest_hook::libil2cpp::ByRefMut<crate::System::Nullable_1<i64>>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonReader,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::System::Nullable_1<i64>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetDateConstructorValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryGetDateConstructorValue", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (reader, integer, errorMessage))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateFromConstructorJson(
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        dateTime: quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonReader,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::System::DateTime>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetDateFromConstructorJson")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "TryGetDateFromConstructorJson", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (reader, dateTime, errorMessage))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            char,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        4usize,
                    >("WriteCharAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteCharAsync", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method.invoke_unchecked((), (task, writer, c, cancellationToken))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        8usize,
                    >("WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::IArrayPool_1<char>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<char>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("WriteEscapedJavaScriptString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteEscapedJavaScriptString", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        9usize,
                    >("WriteEscapedJavaScriptStringAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "WriteEscapedJavaScriptStringAsync", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        9usize,
                    >("WriteEscapedJavaScriptStringWithDelimitersAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "WriteEscapedJavaScriptStringWithDelimitersAsync", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        8usize,
                    >("WriteEscapedJavaScriptStringWithDelimitersAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "WriteEscapedJavaScriptStringWithDelimitersAsync", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<bool>,
                            >,
                            crate::Newtonsoft::Json::StringEscapeHandling,
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<char>,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        7usize,
                    >("WriteEscapedJavaScriptStringWithoutDelimitersAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "WriteEscapedJavaScriptStringWithoutDelimitersAsync", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        writer,
                        s,
                        charEscapeFlags,
                        stringEscapeHandling,
                        client,
                        writeBuffer,
                        cancellationToken,
                    ),
                )?
        };
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

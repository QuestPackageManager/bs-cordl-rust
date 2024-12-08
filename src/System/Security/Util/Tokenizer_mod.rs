#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer_ITokenReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer_ITokenReader
    => "System.Security.Util"."Tokenizer/ITokenReader"
);
#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer_ITokenReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
impl std::ops::DerefMut for crate::System::Security::Util::Tokenizer_ITokenReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
impl crate::System::Security::Util::Tokenizer_ITokenReader {
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Util::Tokenizer_ITokenReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer_StreamTokenReader {
    __cordl_parent: crate::System::Object,
    pub _in: *mut crate::System::IO::StreamReader,
    pub _numCharRead: i32,
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Util::Tokenizer_StreamTokenReader => "System.Security.Util"
    ."Tokenizer/StreamTokenReader"
);
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer_StreamTokenReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl std::ops::DerefMut for crate::System::Security::Util::Tokenizer_StreamTokenReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl crate::System::Security::Util::Tokenizer_StreamTokenReader {
    pub fn New(
        input: *mut crate::System::IO::StreamReader,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        input: *mut crate::System::IO::StreamReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
    pub fn get_NumCharEncountered(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NumCharEncountered", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Util::Tokenizer_StreamTokenReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer_StringMaker {
    __cordl_parent: crate::System::Object,
    pub aStrings: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub cStringsMax: u32,
    pub cStringsUsed: u32,
    pub _outStringBuilder: *mut crate::System::Text::StringBuilder,
    pub _outChars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _outIndex: i32,
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer_StringMaker =>
    "System.Security.Util"."Tokenizer/StringMaker"
);
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer_StringMaker {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
impl std::ops::DerefMut for crate::System::Security::Util::Tokenizer_StringMaker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
impl crate::System::Security::Util::Tokenizer_StringMaker {
    pub fn CompareStringAndChars(
        &mut self,
        str: *mut crate::System::String,
        a: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareStringAndChars", (str, a, l))?;
        Ok(__cordl_ret)
    }
    pub fn MakeString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("MakeString", ())?;
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
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Util::Tokenizer_StringMaker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+TokenSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tokenizer_TokenSource {
    ASCIIByteArray = 2i32,
    CharArray = 3i32,
    NestedStrings = 5i32,
    Other = 6i32,
    String = 4i32,
    UTF8ByteArray = 1i32,
    UnicodeByteArray = 0i32,
}
#[cfg(feature = "System+Security+Util+Tokenizer+TokenSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer_TokenSource =>
    "System.Security.Util"."Tokenizer/TokenSource"
);
#[cfg(feature = "System+Security+Util+Tokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer {
    __cordl_parent: crate::System::Object,
    pub LineNo: i32,
    pub _inProcessingTag: i32,
    pub _inBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _inChars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _inString: *mut crate::System::String,
    pub _inIndex: i32,
    pub _inSize: i32,
    pub _inSavedCharacter: i32,
    pub _inTokenSource: crate::System::Security::Util::Tokenizer_TokenSource,
    pub _inTokenReader: *mut crate::System::Security::Util::Tokenizer_ITokenReader,
    pub _maker: *mut crate::System::Security::Util::Tokenizer_StringMaker,
    pub _searchStrings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _replaceStrings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _inNestedIndex: i32,
    pub _inNestedSize: i32,
    pub _inNestedString: *mut crate::System::String,
}
#[cfg(feature = "System+Security+Util+Tokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer =>
    "System.Security.Util"."Tokenizer"
);
#[cfg(feature = "System+Security+Util+Tokenizer")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer")]
impl std::ops::DerefMut for crate::System::Security::Util::Tokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer")]
impl crate::System::Security::Util::Tokenizer {
    #[cfg(feature = "System+Security+Util+Tokenizer+ITokenReader")]
    type ITokenReader = crate::System::Security::Util::Tokenizer_ITokenReader;
    #[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
    pub type StreamTokenReader = crate::System::Security::Util::Tokenizer_StreamTokenReader;
    #[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
    pub type StringMaker = crate::System::Security::Util::Tokenizer_StringMaker;
    #[cfg(feature = "System+Security+Util+Tokenizer+TokenSource")]
    pub type TokenSource = crate::System::Security::Util::Tokenizer_TokenSource;
    pub fn BasicInitialization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BasicInitialization", ())?;
        Ok(__cordl_ret)
    }
    pub fn ChangeFormat(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeFormat", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTokens(
        &mut self,
        stream: *mut crate::System::Security::Util::TokenizerStream,
        maxNum: i32,
        endAfterKet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTokens", (stream, maxNum, endAfterKet))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        input: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object)
    }
    pub fn Recycle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Recycle", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        input: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::Util::Tokenizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

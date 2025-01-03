#[cfg(feature = "System+Security+Util+Tokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub LineNo: i32,
    pub _inProcessingTag: i32,
    pub _inBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _inChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _inString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _inIndex: i32,
    pub _inSize: i32,
    pub _inSavedCharacter: i32,
    pub _inTokenSource: crate::System::Security::Util::Tokenizer_TokenSource,
    pub _inTokenReader: quest_hook::libil2cpp::Gc<
        crate::System::Security::Util::Tokenizer_ITokenReader,
    >,
    pub _maker: quest_hook::libil2cpp::Gc<
        crate::System::Security::Util::Tokenizer_StringMaker,
    >,
    pub _searchStrings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _replaceStrings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _inNestedIndex: i32,
    pub _inNestedSize: i32,
    pub _inNestedString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Security+Util+Tokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer =>
    "System.Security.Util"."Tokenizer"
);
#[cfg(feature = "System+Security+Util+Tokenizer")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn ChangeFormat(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeFormat", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStringToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTokens(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<
            crate::System::Security::Util::TokenizerStream,
        >,
        maxNum: i32,
        endAfterKet: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTokens", (stream, maxNum, endAfterKet))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object.into())
    }
    pub fn Recycle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Recycle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _in: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        input: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumCharEncountered(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NumCharEncountered", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl AsRef<crate::System::Security::Util::Tokenizer_ITokenReader>
for crate::System::Security::Util::Tokenizer_StreamTokenReader {
    fn as_ref(&self) -> &crate::System::Security::Util::Tokenizer_ITokenReader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StreamTokenReader")]
impl AsMut<crate::System::Security::Util::Tokenizer_ITokenReader>
for crate::System::Security::Util::Tokenizer_StreamTokenReader {
    fn as_mut(&mut self) -> &mut crate::System::Security::Util::Tokenizer_ITokenReader {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
#[repr(C)]
#[derive(Debug)]
pub struct Tokenizer_StringMaker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub aStrings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub cStringsMax: u32,
    pub cStringsUsed: u32,
    pub _outStringBuilder: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _outChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _outIndex: i32,
}
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Util::Tokenizer_StringMaker =>
    "System.Security.Util"."Tokenizer/StringMaker"
);
#[cfg(feature = "System+Security+Util+Tokenizer+StringMaker")]
impl std::ops::Deref for crate::System::Security::Util::Tokenizer_StringMaker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CompareStringAndChars", (str, a, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashCharArray(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        l: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashCharArray", (a, l))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashString(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashString", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("MakeString", ())?;
        Ok(__cordl_ret.into())
    }
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

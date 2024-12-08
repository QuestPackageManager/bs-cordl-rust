#[cfg(feature = "System+TermInfoDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct TermInfoDriver {
    __cordl_parent: crate::System::Object,
    pub reader: *mut crate::System::TermInfoReader,
    pub cursorLeft: i32,
    pub cursorTop: i32,
    pub title: *mut crate::System::String,
    pub titleFormat: *mut crate::System::String,
    pub cursorVisible: bool,
    pub csrVisible: *mut crate::System::String,
    pub csrInvisible: *mut crate::System::String,
    pub clear: *mut crate::System::String,
    pub bell: *mut crate::System::String,
    pub term: *mut crate::System::String,
    pub stdin: *mut crate::System::IO::StreamReader,
    pub stdout: *mut crate::System::IO::CStreamWriter,
    pub windowWidth: i32,
    pub windowHeight: i32,
    pub bufferHeight: i32,
    pub bufferWidth: i32,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub readpos: i32,
    pub writepos: i32,
    pub keypadXmit: *mut crate::System::String,
    pub keypadLocal: *mut crate::System::String,
    pub inited: bool,
    pub initLock: *mut crate::System::Object,
    pub initKeys: bool,
    pub origPair: *mut crate::System::String,
    pub origColors: *mut crate::System::String,
    pub cursorAddress: *mut crate::System::String,
    pub fgcolor: crate::System::ConsoleColor,
    pub setfgcolor: *mut crate::System::String,
    pub setbgcolor: *mut crate::System::String,
    pub maxColors: i32,
    pub noGetPosition: bool,
    pub keymap: *mut crate::System::Collections::Hashtable,
    pub rootmap: *mut crate::System::ByteMatcher,
    pub rl_startx: i32,
    pub rl_starty: i32,
    pub control_characters: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub echobuf: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub echon: i32,
}
#[cfg(feature = "System+TermInfoDriver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::TermInfoDriver => "System"
    ."TermInfoDriver"
);
#[cfg(feature = "System+TermInfoDriver")]
impl std::ops::Deref for crate::System::TermInfoDriver {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+TermInfoDriver")]
impl std::ops::DerefMut for crate::System::TermInfoDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+TermInfoDriver")]
impl crate::System::TermInfoDriver {
    pub fn InitKeys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitKeys", ())?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        dest: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (dest, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn IsSpecialKey_ConsoleKeyInfo0(
        &mut self,
        key: crate::System::ConsoleKeyInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSpecialKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn IsSpecialKey__cordl_char1(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSpecialKey", (c))?;
        Ok(__cordl_ret)
    }
    pub fn get_WindowWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WindowWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadLine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadLine", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteSpecialKey_ConsoleKeyInfo0(
        &mut self,
        key: crate::System::ConsoleKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSpecialKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn WriteSpecialKey__cordl_char1(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteSpecialKey", (c))?;
        Ok(__cordl_ret)
    }
    pub fn CreateKeyInfoFromInt(
        &mut self,
        n: i32,
        alt: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ConsoleKeyInfo = __cordl_object
            .invoke("CreateKeyInfoFromInt", (n, alt))?;
        Ok(__cordl_ret)
    }
    pub fn ReadKeyInternal(
        &mut self,
        fresh: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ConsoleKeyInfo = __cordl_object
            .invoke("ReadKeyInternal", (fresh))?;
        Ok(__cordl_ret)
    }
    pub fn InputPending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InputPending", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetKeyFromBuffer(
        &mut self,
        cooked: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetKeyFromBuffer", (cooked))?;
        Ok(__cordl_ret)
    }
    pub fn ReadKey(
        &mut self,
        intercept: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::ConsoleKeyInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ConsoleKeyInfo = __cordl_object
            .invoke("ReadKey", (intercept))?;
        Ok(__cordl_ret)
    }
    pub fn WriteConsole(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteConsole", (str))?;
        Ok(__cordl_ret)
    }
    pub fn GetCursorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCursorPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WindowHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WindowHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Initialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Initialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn Echo(
        &mut self,
        key: crate::System::ConsoleKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Echo", (key))?;
        Ok(__cordl_ret)
    }
    pub fn EchoFlush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EchoFlush", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementX", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckWindowDimensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckWindowDimensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadToEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadToEnd", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCursorPosition(
        &mut self,
        left: i32,
        top: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCursorPosition", (left, top))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        term: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (term))?;
        Ok(__cordl_ret)
    }
    pub fn ReadUntilConditionInternal(
        &mut self,
        haltOnNewLine: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadUntilConditionInternal", (haltOnNewLine))?;
        Ok(__cordl_ret)
    }
    pub fn AddStringMapping(
        &mut self,
        s: crate::System::TermInfoStrings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStringMapping", (s))?;
        Ok(__cordl_ret)
    }
    pub fn AdjustBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddToBuffer(
        &mut self,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToBuffer", (b))?;
        Ok(__cordl_ret)
    }
    pub fn QueueEcho(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEcho", (c))?;
        Ok(__cordl_ret)
    }
    pub fn CreateKeyMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateKeyMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        term: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (term))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+TermInfoDriver")]
impl quest_hook::libil2cpp::ObjectType for crate::System::TermInfoDriver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

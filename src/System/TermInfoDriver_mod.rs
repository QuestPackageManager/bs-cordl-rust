#[cfg(feature = "System+TermInfoDriver")]
#[repr(C)]
#[derive(Debug)]
pub struct TermInfoDriver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub reader: *mut crate::System::TermInfoReader,
    pub cursorLeft: i32,
    pub cursorTop: i32,
    pub title: *mut quest_hook::libil2cpp::Il2CppString,
    pub titleFormat: *mut quest_hook::libil2cpp::Il2CppString,
    pub cursorVisible: bool,
    pub csrVisible: *mut quest_hook::libil2cpp::Il2CppString,
    pub csrInvisible: *mut quest_hook::libil2cpp::Il2CppString,
    pub clear: *mut quest_hook::libil2cpp::Il2CppString,
    pub bell: *mut quest_hook::libil2cpp::Il2CppString,
    pub term: *mut quest_hook::libil2cpp::Il2CppString,
    pub stdin: *mut crate::System::IO::StreamReader,
    pub stdout: *mut crate::System::IO::CStreamWriter,
    pub windowWidth: i32,
    pub windowHeight: i32,
    pub bufferHeight: i32,
    pub bufferWidth: i32,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub readpos: i32,
    pub writepos: i32,
    pub keypadXmit: *mut quest_hook::libil2cpp::Il2CppString,
    pub keypadLocal: *mut quest_hook::libil2cpp::Il2CppString,
    pub inited: bool,
    pub initLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub initKeys: bool,
    pub origPair: *mut quest_hook::libil2cpp::Il2CppString,
    pub origColors: *mut quest_hook::libil2cpp::Il2CppString,
    pub cursorAddress: *mut quest_hook::libil2cpp::Il2CppString,
    pub fgcolor: crate::System::ConsoleColor,
    pub setfgcolor: *mut quest_hook::libil2cpp::Il2CppString,
    pub setbgcolor: *mut quest_hook::libil2cpp::Il2CppString,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddStringMapping(
        &mut self,
        s: crate::System::TermInfoStrings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStringMapping", (s))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn AdjustBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckWindowDimensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckWindowDimensions", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateKeyMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateKeyMap", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn EchoFlush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EchoFlush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCursorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCursorPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyFromBuffer(
        &mut self,
        cooked: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetKeyFromBuffer", (cooked))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncrementX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitKeys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitKeys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InputPending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InputPending", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSpecialKey_ConsoleKeyInfo0(
        &mut self,
        key: crate::System::ConsoleKeyInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSpecialKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSpecialKey__cordl_char1(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSpecialKey", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        term: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (term))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ReadLine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadLine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadToEnd(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadToEnd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUntilConditionInternal(
        &mut self,
        haltOnNewLine: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadUntilConditionInternal", (haltOnNewLine))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchTerminfo(
        term: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchTerminfo", (term))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryTermInfoDir(
        dir: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        term: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryTermInfoDir", (dir, term))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteConsole(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteConsole", (str))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        term: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (term))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Initialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Initialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WindowHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WindowHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WindowWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WindowWidth", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+TermInfoDriver")]
impl AsRef<crate::System::IConsoleDriver> for crate::System::TermInfoDriver {
    fn as_ref(&self) -> &crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+TermInfoDriver")]
impl AsMut<crate::System::IConsoleDriver> for crate::System::TermInfoDriver {
    fn as_mut(&mut self) -> &mut crate::System::IConsoleDriver {
        unsafe { std::mem::transmute(self) }
    }
}

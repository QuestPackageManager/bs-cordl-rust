#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexParser {
    __cordl_parent: crate::System::Object,
    pub _stack: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub _group: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub _alternation: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub _concatenation: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub _unit: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub _pattern: *mut crate::System::String,
    pub _currentPos: i32,
    pub _culture: *mut crate::System::Globalization::CultureInfo,
    pub _autocap: i32,
    pub _capcount: i32,
    pub _captop: i32,
    pub _capsize: i32,
    pub _caps: *mut crate::System::Collections::Hashtable,
    pub _capnames: *mut crate::System::Collections::Hashtable,
    pub _capnumlist: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _capnamelist: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _options: crate::System::Text::RegularExpressions::RegexOptions,
    pub _optionsStack: *mut crate::System::Collections::Generic::List_1<
        crate::System::Text::RegularExpressions::RegexOptions,
    >,
    pub _ignoreNextParen: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexParser =>
    "System.Text.RegularExpressions"."RegexParser"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
impl crate::System::Text::RegularExpressions::RegexParser {
    pub fn AddAlternate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAlternate", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddConcatenate_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddConcatenate", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddConcatenate__cordl_bool_i32_i32_2(
        &mut self,
        lazy: bool,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddConcatenate", (lazy, min, max))?;
        Ok(__cordl_ret)
    }
    pub fn AddConcatenate_i32_i32__cordl_bool0(
        &mut self,
        pos: i32,
        cch: i32,
        isReplacement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddConcatenate", (pos, cch, isReplacement))?;
        Ok(__cordl_ret)
    }
    pub fn AddGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddUnitNode(
        &mut self,
        node: *mut crate::System::Text::RegularExpressions::RegexNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUnitNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn AddUnitNotone(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUnitNotone", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn AddUnitOne(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUnitOne", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn AddUnitSet(
        &mut self,
        cc: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUnitSet", (cc))?;
        Ok(__cordl_ret)
    }
    pub fn AddUnitType(
        &mut self,
        _cordl_type: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUnitType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn AssignNameSlots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignNameSlots", ())?;
        Ok(__cordl_ret)
    }
    pub fn CaptureSlotFromName(
        &mut self,
        capname: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CaptureSlotFromName", (capname))?;
        Ok(__cordl_ret)
    }
    pub fn CharAt(&mut self, i: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("CharAt", (i))?;
        Ok(__cordl_ret)
    }
    pub fn CharsRight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CharsRight", ())?;
        Ok(__cordl_ret)
    }
    pub fn CountCaptures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CountCaptures", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmptyOptionsStack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EmptyOptionsStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn EmptyStack(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EmptyStack", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsCaptureName(
        &mut self,
        capname: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCaptureName", (capname))?;
        Ok(__cordl_ret)
    }
    pub fn IsCaptureSlot(&mut self, i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCaptureSlot", (i))?;
        Ok(__cordl_ret)
    }
    pub fn IsOnlyTopOption(
        &mut self,
        option: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsOnlyTopOption", (option))?;
        Ok(__cordl_ret)
    }
    pub fn IsTrueQuantifier(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTrueQuantifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn MakeException(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::ArgumentException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ArgumentException = __cordl_object
            .invoke("MakeException", (message))?;
        Ok(__cordl_ret)
    }
    pub fn MoveLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveLeft", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveRight_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveRight", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveRight_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveRight", (i))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object)
    }
    pub fn NoteCaptureName(
        &mut self,
        name: *mut crate::System::String,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteCaptureName", (name, pos))?;
        Ok(__cordl_ret)
    }
    pub fn NoteCaptureSlot(
        &mut self,
        i: i32,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteCaptureSlot", (i, pos))?;
        Ok(__cordl_ret)
    }
    pub fn NoteCaptures(
        &mut self,
        caps: *mut crate::System::Collections::Hashtable,
        capsize: i32,
        capnames: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteCaptures", (caps, capsize, capnames))?;
        Ok(__cordl_ret)
    }
    pub fn ParseProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParseProperty", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopKeepOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopKeepOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushGroup", ())?;
        Ok(__cordl_ret)
    }
    pub fn PushOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
        topopts: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (topopts))?;
        Ok(__cordl_ret)
    }
    pub fn RightCharMoveRight(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("RightCharMoveRight", ())?;
        Ok(__cordl_ret)
    }
    pub fn RightChar_0(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("RightChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn RightChar_i32_1(&mut self, i: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("RightChar", (i))?;
        Ok(__cordl_ret)
    }
    pub fn ScanBackslash(
        &mut self,
        scanOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanBackslash", (scanOnly))?;
        Ok(__cordl_ret)
    }
    pub fn ScanBasicBackslash(
        &mut self,
        scanOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanBasicBackslash", (scanOnly))?;
        Ok(__cordl_ret)
    }
    pub fn ScanBlank(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanBlank", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanCapname(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ScanCapname", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanCharClass(
        &mut self,
        caseInsensitive: bool,
        scanOnly: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexCharClass,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexCharClass = __cordl_object
            .invoke("ScanCharClass", (caseInsensitive, scanOnly))?;
        Ok(__cordl_ret)
    }
    pub fn ScanCharEscape(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ScanCharEscape", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanControl(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ScanControl", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanDecimal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ScanDecimal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanDollar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanDollar", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanGroupOpen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanGroupOpen", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanHex(&mut self, c: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ScanHex", (c))?;
        Ok(__cordl_ret)
    }
    pub fn ScanOctal(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ScanOctal", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanOptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScanOptions", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanRegex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanRegex", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScanReplacement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("ScanReplacement", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetPattern(
        &mut self,
        Re: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPattern", (Re))?;
        Ok(__cordl_ret)
    }
    pub fn StartGroup(
        &mut self,
        openGroup: *mut crate::System::Text::RegularExpressions::RegexNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartGroup", (openGroup))?;
        Ok(__cordl_ret)
    }
    pub fn Textpos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Textpos", ())?;
        Ok(__cordl_ret)
    }
    pub fn Textto(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Textto", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn TypeFromCode(&mut self, ch: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("TypeFromCode", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn Unit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexNode = __cordl_object
            .invoke("Unit", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionE(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionE", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionI(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionI", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionM(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionM", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionN(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionN", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionS(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionS", ())?;
        Ok(__cordl_ret)
    }
    pub fn UseOptionX(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionX", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexParser")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
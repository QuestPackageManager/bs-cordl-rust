#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexInterpreter {
    __cordl_parent: crate::System::Text::RegularExpressions::RegexRunner,
    pub _code: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexCode,
    >,
    pub _culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    pub _operator: i32,
    pub _codepos: i32,
    pub _rightToLeft: bool,
    pub _caseInsensitive: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::RegexInterpreter =>
    "System.Text.RegularExpressions"."RegexInterpreter"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexInterpreter {
    type Target = crate::System::Text::RegularExpressions::RegexRunner;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexInterpreter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
impl crate::System::Text::RegularExpressions::RegexInterpreter {
    pub fn Advance(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Advance", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn Backtrack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Backtrack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Backwardnext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Backwardnext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Bump(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Bump", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CharAt(&mut self, j: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("CharAt", (j))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindFirstChar(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("FindFirstChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Forwardcharnext(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("Forwardcharnext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Forwardchars(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Forwardchars", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Go(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Go", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Goto(
        &mut self,
        newpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Goto", (newpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitTrackCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTrackCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Leftchars(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Leftchars", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        code: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (code, culture))?;
        Ok(__cordl_object.into())
    }
    pub fn Operand(&mut self, i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Operand", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn Operator(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Operator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Refmatch(
        &mut self,
        index: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Refmatch", (index, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rightchars(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Rightchars", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOperator(
        &mut self,
        op: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOperator", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPeek_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("StackPeek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPeek_i32_1(&mut self, i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("StackPeek", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPop_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StackPop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPop_i32_1(
        &mut self,
        framesize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StackPop", (framesize))?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPush_i32_0(
        &mut self,
        I1: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StackPush", (I1))?;
        Ok(__cordl_ret.into())
    }
    pub fn StackPush_i32_1(
        &mut self,
        I1: i32,
        I2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StackPush", (I1, I2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stringmatch(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Stringmatch", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn Textpos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Textpos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Textstart(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Textstart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Textto(
        &mut self,
        newpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Textto", (newpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPeek_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("TrackPeek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPeek_i32_1(&mut self, i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("TrackPeek", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPop_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPop_i32_1(
        &mut self,
        framesize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPop", (framesize))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush2_i32_0(
        &mut self,
        I1: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush2", (I1))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush2_i32_1(
        &mut self,
        I1: i32,
        I2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush2", (I1, I2))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush_i32_1(
        &mut self,
        I1: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush", (I1))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush_i32_i32_2(
        &mut self,
        I1: i32,
        I2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush", (I1, I2))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrackPush_i32_i32_i32_3(
        &mut self,
        I1: i32,
        I2: i32,
        I3: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrackPush", (I1, I2, I3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trackpos(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Trackpos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Trackto(
        &mut self,
        newpos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Trackto", (newpos))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        code: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (code, culture))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexInterpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexInterpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

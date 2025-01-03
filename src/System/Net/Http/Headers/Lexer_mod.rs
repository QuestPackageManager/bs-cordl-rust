#[cfg(feature = "System+Net+Http+Headers+Lexer")]
#[repr(C)]
#[derive(Debug)]
pub struct Lexer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pos: i32,
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Lexer =>
    "System.Net.Http.Headers"."Lexer"
);
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Lexer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::Lexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
impl crate::System::Net::Http::Headers::Lexer {
    pub fn EatChar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EatChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQuotedStringValue(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetQuotedStringValue", (start))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRemainingStringValue(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetRemainingStringValue", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue_Token0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStringValue", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue_Token1(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
        end: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetStringValue", (start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsStarStringValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsStarStringValue", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCharacter(input: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidCharacter", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidToken(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValidToken", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        stream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
        Ok(__cordl_object.into())
    }
    pub fn PeekChar(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("PeekChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Scan(
        &mut self,
        recognizeDash: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Http::Headers::Token> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Http::Headers::Token = __cordl_object
            .invoke("Scan", (recognizeDash))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScanCommentOptional(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
        readToken: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Http::Headers::Token,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ScanCommentOptional", (value, readToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateValue_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetDateValue", (text, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateValue_Token0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetDateValue", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDoubleValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetDoubleValue", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetNumericValue_Token_ByRefMut0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetNumericValue", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetNumericValue_Token_ByRefMut1(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetNumericValue", (token, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetTimeSpanValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::TimeSpan>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::TimeSpan> = __cordl_object
            .invoke("TryGetTimeSpanValue", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Position", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::Headers::Lexer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

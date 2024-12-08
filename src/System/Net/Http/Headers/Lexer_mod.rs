#[cfg(feature = "System+Net+Http+Headers+Lexer")]
#[repr(C)]
#[derive(Debug)]
pub struct Lexer {
    __cordl_parent: crate::System::Object,
    pub s: *mut crate::System::String,
    pub pos: i32,
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::Lexer =>
    "System.Net.Http.Headers"."Lexer"
);
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
impl std::ops::Deref for crate::System::Net::Http::Headers::Lexer {
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn GetQuotedStringValue(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetQuotedStringValue", (start))?;
        Ok(__cordl_ret)
    }
    pub fn GetRemainingStringValue(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetRemainingStringValue", (position))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringValue_Token0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringValue", (token))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringValue_Token1(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
        end: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetStringValue", (start, end))?;
        Ok(__cordl_ret)
    }
    pub fn IsStarStringValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsStarStringValue", (token))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        stream: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (stream))?;
        Ok(__cordl_object)
    }
    pub fn PeekChar(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("PeekChar", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ScanCommentOptional(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
        readToken: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Http::Headers::Token,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ScanCommentOptional", (value, readToken))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDateValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetDateValue", (token, value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        stream: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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

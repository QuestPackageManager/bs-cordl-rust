#[cfg(feature = "System+Net+Http+Headers+Lexer")]
#[repr(C)]
#[derive(Debug)]
pub struct Lexer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub pos: i32,
}
#[cfg(feature = "System+Net+Http+Headers+Lexer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Http::Headers::Lexer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http.Headers";
    const CLASS_NAME: &'static str = "Lexer";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EatChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EatChar", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetQuotedStringValue(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::Http::Headers::Token),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetQuotedStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetQuotedStringValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (start)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemainingStringValue(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetRemainingStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRemainingStringValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (position)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue_Token0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::Http::Headers::Token),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringValue", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (token)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetStringValue_Token1(
        &mut self,
        start: crate::System::Net::Http::Headers::Token,
        end: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::Http::Headers::Token,
                    crate::System::Net::Http::Headers::Token,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStringValue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (start, end)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsStarStringValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::Http::Headers::Token),
                bool,
                1usize,
            >("IsStarStringValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsStarStringValue", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (token)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCharacter(input: char) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(char), bool, 1usize>("IsValidCharacter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValidCharacter", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (input)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidToken(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsValidToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsValidToken", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (input)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("PeekChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PeekChar", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Scan(
        &mut self,
        recognizeDash: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Http::Headers::Token> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                crate::System::Net::Http::Headers::Token,
                1usize,
            >("Scan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Scan", 1usize
                )
            });
        let __cordl_ret: crate::System::Net::Http::Headers::Token = unsafe {
            method.invoke_unchecked(self, (recognizeDash))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScanCommentOptional(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        readToken: quest_hook::libil2cpp::ByRefMut<
            crate::System::Net::Http::Headers::Token,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Net::Http::Headers::Token,
                    >,
                ),
                bool,
                2usize,
            >("ScanCommentOptional")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScanCommentOptional", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (value, readToken))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateValue_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
                ),
                bool,
                2usize,
            >("TryGetDateValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetDateValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (text, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDateValue_Token0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::Http::Headers::Token,
                    quest_hook::libil2cpp::ByRefMut<crate::System::DateTimeOffset>,
                ),
                bool,
                2usize,
            >("TryGetDateValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetDateValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (token, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDoubleValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::Http::Headers::Token,
                    quest_hook::libil2cpp::ByRefMut<f64>,
                ),
                bool,
                2usize,
            >("TryGetDoubleValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetDoubleValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (token, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetNumericValue_Token_ByRefMut0(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::Http::Headers::Token,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                bool,
                2usize,
            >("TryGetNumericValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetNumericValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (token, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetNumericValue_Token_ByRefMut1(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
        value: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Net::Http::Headers::Token,
                    quest_hook::libil2cpp::ByRefMut<i64>,
                ),
                bool,
                2usize,
            >("TryGetNumericValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetNumericValue", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (token, value)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetTimeSpanValue(
        &mut self,
        token: crate::System::Net::Http::Headers::Token,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::TimeSpan>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::Http::Headers::Token),
                crate::System::Nullable_1<crate::System::TimeSpan>,
                1usize,
            >("TryGetTimeSpanValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetTimeSpanValue", 1usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<crate::System::TimeSpan> = unsafe {
            method.invoke_unchecked(self, (token))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        stream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (stream))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Position")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Position", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Position")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_Position", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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

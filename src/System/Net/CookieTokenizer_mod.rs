#[cfg(feature = "System+Net+CookieTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieTokenizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_eofCookie: bool,
    pub m_index: i32,
    pub m_length: i32,
    pub m_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_quoted: bool,
    pub m_start: i32,
    pub m_token: crate::System::Net::CookieToken,
    pub m_tokenLength: i32,
    pub m_tokenStream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+CookieTokenizer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::CookieTokenizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CookieTokenizer";
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
#[cfg(feature = "System+Net+CookieTokenizer")]
impl std::ops::Deref for crate::System::Net::CookieTokenizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieTokenizer")]
impl std::ops::DerefMut for crate::System::Net::CookieTokenizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieTokenizer")]
impl crate::System::Net::CookieTokenizer {
    #[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
    pub type RecognizedAttribute = crate::System::Net::CookieTokenizer_RecognizedAttribute;
    pub fn Extract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("Extract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "Extract", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FindNext(
        &mut self,
        ignoreComma: bool,
        ignoreEquals: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                crate::System::Net::CookieToken,
                2usize,
            >("FindNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "FindNext", 2usize
                )
            });
        let __cordl_ret: crate::System::Net::CookieToken = unsafe {
            method.invoke_unchecked(self, (ignoreComma, ignoreEquals))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tokenStream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tokenStream))?;
        Ok(__cordl_object.into())
    }
    pub fn Next(
        &mut self,
        first: bool,
        parseResponseCookies: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool, bool), crate::System::Net::CookieToken, 2usize>("Next")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "Next", 2usize
                )
            });
        let __cordl_ret: crate::System::Net::CookieToken = unsafe {
            method.invoke_unchecked(self, (first, parseResponseCookies))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TokenFromName(
        &mut self,
        parseResponseCookies: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                crate::System::Net::CookieToken,
                1usize,
            >("TokenFromName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "TokenFromName", 1usize
                )
            });
        let __cordl_ret: crate::System::Net::CookieToken = unsafe {
            method.invoke_unchecked(self, (parseResponseCookies))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        tokenStream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tokenStream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_EndOfCookie(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_EndOfCookie")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_EndOfCookie", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Eof(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Eof")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_Eof", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Name")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_Name", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Quoted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_Quoted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_Quoted", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::Net::CookieToken, 0usize>("get_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_Token", 0usize
                )
            });
        let __cordl_ret: crate::System::Net::CookieToken = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "get_Value", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_EndOfCookie(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_EndOfCookie")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "set_EndOfCookie", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Name")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "set_Name", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Quoted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_Quoted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "set_Quoted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Token(
        &mut self,
        value: crate::System::Net::CookieToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Net::CookieToken),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "set_Token", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer as quest_hook::libil2cpp::Type >
                    ::class(), "set_Value", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+CookieTokenizer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CookieTokenizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookieTokenizer_RecognizedAttribute {
    pub m_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_token: crate::System::Net::CookieToken,
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CookieTokenizer/RecognizedAttribute";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::CookieTokenizer_RecognizedAttribute {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
impl crate::System::Net::CookieTokenizer_RecognizedAttribute {
    pub fn IsEqualTo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer_RecognizedAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsEqualTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer_RecognizedAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "IsEqualTo", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Net::CookieToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer_RecognizedAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Net::CookieToken,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer_RecognizedAttribute as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, token))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieTokenizer_RecognizedAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::Net::CookieToken, 0usize>("get_Token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieTokenizer_RecognizedAttribute as
                    quest_hook::libil2cpp::Type > ::class(), "get_Token", 0usize
                )
            });
        let __cordl_ret: crate::System::Net::CookieToken = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}

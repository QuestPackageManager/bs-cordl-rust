#[cfg(feature = "System+Net+CookieTokenizer")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieTokenizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_eofCookie: bool,
    pub m_index: i32,
    pub m_length: i32,
    pub m_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_quoted: bool,
    pub m_start: i32,
    pub m_token: crate::System::Net::CookieToken,
    pub m_tokenLength: i32,
    pub m_tokenStream: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_value: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Net+CookieTokenizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieTokenizer => "System.Net"
    ."CookieTokenizer"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Extract", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindNext(
        &mut self,
        ignoreComma: bool,
        ignoreEquals: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CookieToken = __cordl_object
            .invoke("FindNext", (ignoreComma, ignoreEquals))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CookieToken = __cordl_object
            .invoke("Next", (first, parseResponseCookies))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TokenFromName(
        &mut self,
        parseResponseCookies: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CookieToken = __cordl_object
            .invoke("TokenFromName", (parseResponseCookies))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        tokenStream: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tokenStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EndOfCookie(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_EndOfCookie", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Eof(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Eof", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Quoted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Quoted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CookieToken = __cordl_object
            .invoke("get_Token", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_EndOfCookie(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EndOfCookie", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Quoted(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Quoted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Token(
        &mut self,
        value: crate::System::Net::CookieToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Token", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Value", (value))?;
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
#[derive(Debug, Clone)]
pub struct CookieTokenizer_RecognizedAttribute {
    pub m_name: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_token: crate::System::Net::CookieToken,
}
#[cfg(feature = "System+Net+CookieTokenizer+RecognizedAttribute")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieTokenizer_RecognizedAttribute
    => "System.Net"."CookieTokenizer/RecognizedAttribute"
);
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
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsEqualTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        token: crate::System::Net::CookieToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, token),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::CookieToken> {
        let __cordl_ret: crate::System::Net::CookieToken = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Token",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}

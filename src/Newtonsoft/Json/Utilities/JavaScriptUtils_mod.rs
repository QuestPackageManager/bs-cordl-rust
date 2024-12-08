#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct JavaScriptUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::JavaScriptUtils =>
    "Newtonsoft.Json.Utilities"."JavaScriptUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    pub const EscapedUnicodeText: &'static str = "!";
    pub const UnicodeTextLength: i32 = 6i32;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+JavaScriptUtils+_WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync_d__16"
    )]
    pub type _WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync_d__16 = crate::Newtonsoft::Json::Utilities::JavaScriptUtils__WriteDefinitelyEscapedJavaScriptStringWithoutDelimitersAsync_d__16;
    #[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils+_WriteCharAsync_d__14")]
    pub type _WriteCharAsync_d__14 = crate::Newtonsoft::Json::Utilities::JavaScriptUtils__WriteCharAsync_d__14;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+JavaScriptUtils+_WriteEscapedJavaScriptStringWithDelimitersAsync_d__13"
    )]
    pub type _WriteEscapedJavaScriptStringWithDelimitersAsync_d__13 = crate::Newtonsoft::Json::Utilities::JavaScriptUtils__WriteEscapedJavaScriptStringWithDelimitersAsync_d__13;
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JavaScriptUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::JavaScriptUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

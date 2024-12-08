#[cfg(feature = "System+Net+CookieParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieParser {
    __cordl_parent: crate::System::Object,
    pub m_tokenizer: *mut crate::System::Net::CookieTokenizer,
}
#[cfg(feature = "System+Net+CookieParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieParser => "System.Net"
    ."CookieParser"
);
#[cfg(feature = "System+Net+CookieParser")]
impl std::ops::Deref for crate::System::Net::CookieParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieParser")]
impl std::ops::DerefMut for crate::System::Net::CookieParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieParser")]
impl crate::System::Net::CookieParser {
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Cookie> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Cookie = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        cookieString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cookieString))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        cookieString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cookieString))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+CookieParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CookieParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

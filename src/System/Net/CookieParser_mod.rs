#[cfg(feature = "System+Net+CookieParser")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_tokenizer: quest_hook::libil2cpp::Gc<crate::System::Net::CookieTokenizer>,
}
#[cfg(feature = "System+Net+CookieParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieParser => "System.Net"
    ."CookieParser"
);
#[cfg(feature = "System+Net+CookieParser")]
impl std::ops::Deref for crate::System::Net::CookieParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CheckQuoted(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckQuoted", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Cookie>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::Cookie> = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cookieString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cookieString))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        cookieString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cookieString))?;
        Ok(__cordl_ret.into())
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

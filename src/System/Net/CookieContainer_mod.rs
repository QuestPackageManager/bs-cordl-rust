#[cfg(feature = "System+Net+CookieContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieContainer {
    __cordl_parent: crate::System::Object,
    pub m_domainTable: *mut crate::System::Collections::Hashtable,
    pub m_maxCookieSize: i32,
    pub m_maxCookies: i32,
    pub m_maxCookiesPerDomain: i32,
    pub m_count: i32,
    pub m_fqdnMyDomain: *mut crate::System::String,
}
#[cfg(feature = "System+Net+CookieContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieContainer => "System.Net"
    ."CookieContainer"
);
#[cfg(feature = "System+Net+CookieContainer")]
impl std::ops::Deref for crate::System::Net::CookieContainer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieContainer")]
impl std::ops::DerefMut for crate::System::Net::CookieContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CookieContainer")]
impl crate::System::Net::CookieContainer {
    pub fn IsLocalDomain(
        &mut self,
        host: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLocalDomain", (host))?;
        Ok(__cordl_ret)
    }
    pub fn MergeUpdateCollections(
        &mut self,
        destination: *mut crate::System::Net::CookieCollection,
        source: *mut crate::System::Net::CookieCollection,
        port: i32,
        isSecure: bool,
        isPlainOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MergeUpdateCollections",
                (destination, source, port, isSecure, isPlainOnly),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddRemoveDomain(
        &mut self,
        key: *mut crate::System::String,
        value: *mut crate::System::Net::PathList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRemoveDomain", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        cookie: *mut crate::System::Net::Cookie,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (cookie, throwOnError))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgeCookies(
        &mut self,
        domain: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgeCookies", (domain))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetCookies(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::CookieCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::CookieCollection = __cordl_object
            .invoke("InternalGetCookies", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn BuildCookieCollectionFromDomainMatches(
        &mut self,
        uri: *mut crate::System::Uri,
        isSecure: bool,
        port: i32,
        cookies: *mut crate::System::Net::CookieCollection,
        domainAttribute: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        matchOnlyPlainCookie: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BuildCookieCollectionFromDomainMatches",
                (uri, isSecure, port, cookies, domainAttribute, matchOnlyPlainCookie),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ExpireCollection(
        &mut self,
        cc: *mut crate::System::Net::CookieCollection,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ExpireCollection", (cc))?;
        Ok(__cordl_ret)
    }
    pub fn CookieCutter(
        &mut self,
        uri: *mut crate::System::Uri,
        headerName: *mut crate::System::String,
        setCookieHeader: *mut crate::System::String,
        isThrow: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::CookieCollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::CookieCollection = __cordl_object
            .invoke("CookieCutter", (uri, headerName, setCookieHeader, isThrow))?;
        Ok(__cordl_ret)
    }
    pub fn GetCookieHeader_Uri0(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCookieHeader", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn GetCookieHeader_ByRefMut1(
        &mut self,
        uri: *mut crate::System::Uri,
        optCookie2: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCookieHeader", (uri, optCookie2))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+CookieContainer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CookieContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

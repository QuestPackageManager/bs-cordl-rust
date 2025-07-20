#[cfg(feature = "System+Net+CookieContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct CookieContainer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_domainTable: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub m_maxCookieSize: i32,
    pub m_maxCookies: i32,
    pub m_maxCookiesPerDomain: i32,
    pub m_count: i32,
    pub m_fqdnMyDomain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+CookieContainer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::CookieContainer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CookieContainer";
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
#[cfg(feature = "System+Net+CookieContainer")]
impl std::ops::Deref for crate::System::Net::CookieContainer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Add(
        &mut self,
        cookie: quest_hook::libil2cpp::Gc<crate::System::Net::Cookie>,
        throwOnError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::Cookie>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cookie, throwOnError))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddRemoveDomain(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::PathList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::PathList>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("AddRemoveDomain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "AddRemoveDomain", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AgeCookies(
        &mut self,
        domain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("AgeCookies")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "AgeCookies", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (domain))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildCookieCollectionFromDomainMatches(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        isSecure: bool,
        port: i32,
        cookies: quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
        domainAttribute: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        matchOnlyPlainCookie: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    bool,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("BuildCookieCollectionFromDomainMatches")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "BuildCookieCollectionFromDomainMatches", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (uri, isSecure, port, cookies, domainAttribute, matchOnlyPlainCookie),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CookieCutter(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        headerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        setCookieHeader: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isThrow: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
                4usize,
            >("CookieCutter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "CookieCutter", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::CookieCollection,
        > = unsafe {
            method.invoke_unchecked(self, (uri, headerName, setCookieHeader, isThrow))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExpireCollection(
        &mut self,
        cc: quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>),
                i32,
                1usize,
            >("ExpireCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "ExpireCollection", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (cc))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCookieHeader_ByRefMut1(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        optCookie2: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetCookieHeader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "GetCookieHeader", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (uri, optCookie2))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCookieHeader_Uri0(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetCookieHeader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "GetCookieHeader", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetCookies(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
                1usize,
            >("InternalGetCookies")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "InternalGetCookies", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::CookieCollection,
        > = unsafe { method.invoke_unchecked(self, (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalDomain(
        &mut self,
        host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsLocalDomain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "IsLocalDomain", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (host))? };
        Ok(__cordl_ret.into())
    }
    pub fn MergeUpdateCollections(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
        source: quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
        port: i32,
        isSecure: bool,
        isPlainOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
                    quest_hook::libil2cpp::Gc<crate::System::Net::CookieCollection>,
                    i32,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("MergeUpdateCollections")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), "MergeUpdateCollections", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (destination, source, port, isSecure, isPlainOnly),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Net::CookieContainer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Net::CookieContainer as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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

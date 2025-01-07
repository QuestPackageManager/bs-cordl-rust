#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngineInternal::WebRequestUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngineInternal";
    const CLASS_NAME: &'static str = "WebRequestUtils";
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
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl std::ops::Deref for crate::UnityEngineInternal::WebRequestUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl std::ops::DerefMut for crate::UnityEngineInternal::WebRequestUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl crate::UnityEngineInternal::WebRequestUtils {
    pub fn MakeInitialUrl(
        targetUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeInitialUrl", (targetUrl, localUrl))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeUriString(
        targetUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        targetUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        prependProtocol: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeUriString", (targetUri, targetUrl, prependProtocol))?;
        Ok(__cordl_ret.into())
    }
    pub fn RedirectTo(
        baseUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        redirectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RedirectTo", (baseUri, redirectUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn URLDecode(
        encoded: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("URLDecode", (encoded))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngineInternal+WebRequestUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngineInternal::WebRequestUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoWebProxyScriptEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::AutoWebProxyScriptEngine =>
    "System.Net"."AutoWebProxyScriptEngine"
);
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl std::ops::Deref for crate::System::Net::AutoWebProxyScriptEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl std::ops::DerefMut for crate::System::Net::AutoWebProxyScriptEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl crate::System::Net::AutoWebProxyScriptEngine {
    pub fn GetProxies_ByRefMut1(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxyList: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::IList_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        syncStatus: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetProxies", (destination, proxyList, syncStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProxies_Uri_ByRefMut0(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxyList: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::IList_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetProxies", (destination, proxyList))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::AutoWebProxyScriptEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

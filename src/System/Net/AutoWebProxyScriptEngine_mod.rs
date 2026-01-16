#[cfg(feature = "cordl_class_System+Net+AutoWebProxyScriptEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct AutoWebProxyScriptEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Net+AutoWebProxyScriptEngine")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::AutoWebProxyScriptEngine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "AutoWebProxyScriptEngine";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl std::ops::Deref for crate::System::Net::AutoWebProxyScriptEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl std::ops::DerefMut for crate::System::Net::AutoWebProxyScriptEngine {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+AutoWebProxyScriptEngine")]
impl crate::System::Net::AutoWebProxyScriptEngine {
    pub fn GetProxies_ByRefMut1(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxyList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
        syncStatus: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 3usize>("GetProxies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProxies",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (destination, proxyList, syncStatus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProxies_Uri_ByRefMut0(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxyList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IList_1<
                                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                                >,
                            >,
                        >,
                    ), bool, 2usize>("GetProxies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProxies",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (destination, proxyList))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Net+AutoWebProxyScriptEngine")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::AutoWebProxyScriptEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

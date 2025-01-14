#[cfg(feature = "System+Net+WebProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct WebProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _UseRegistry: bool,
    pub _BypassOnLocal: bool,
    pub m_EnableAutoproxy: bool,
    pub _ProxyAddress: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub _BypassList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub _Credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    pub _RegExBypassList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        >,
    >,
    pub _ProxyHostAddresses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub m_ScriptEngine: quest_hook::libil2cpp::Gc<
        crate::System::Net::AutoWebProxyScriptEngine,
    >,
}
#[cfg(feature = "System+Net+WebProxy")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::WebProxy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "WebProxy";
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
#[cfg(feature = "System+Net+WebProxy")]
impl std::ops::Deref for crate::System::Net::WebProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl std::ops::DerefMut for crate::System::Net::WebProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl crate::System::Net::WebProxy {
    pub fn AreAllBypassed(
        proxies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        checkFirstOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    bool,
                ),
                bool,
                2usize,
            >("AreAllBypassed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AreAllBypassed", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (proxies, checkFirstOnly))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultProxy() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy>,
                0usize,
            >("CreateDefaultProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateDefaultProxy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IWebProxy> = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetObjectData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProxy(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                quest_hook::libil2cpp::Gc<crate::System::Uri>,
                1usize,
            >("GetProxy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetProxy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = unsafe {
            method.invoke_unchecked(self, (destination))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProxyAuto(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        proxyUri: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Uri>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    >,
                ),
                bool,
                2usize,
            >("GetProxyAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetProxyAuto", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (destination, proxyUri))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBypassed(
        &mut self,
        host: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsBypassed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsBypassed", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (host)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsBypassedAuto(
        &mut self,
        destination: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        isBypassed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                2usize,
            >("IsBypassedAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsBypassedAuto", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (destination, isBypassed))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsBypassedManual(
        &mut self,
        host: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsBypassedManual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsBypassedManual", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (host)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsLocal(
        &mut self,
        host: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsLocal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLocal", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (host)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalInProxyHash(
        &mut self,
        host: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsLocalInProxyHash")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLocalInProxyHash", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (host)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsMatchInBypassList(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsMatchInBypassList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsMatchInBypassList", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (input)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri__cordl_bool_Il2CppArray_ICredentials1(
        Address: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        BypassOnLocal: bool,
        BypassList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        Credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (Address, BypassOnLocal, BypassList, Credentials))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool3(
        enableAutoproxy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (enableAutoproxy))?;
        Ok(__cordl_object.into())
    }
    pub fn ProxyUri(
        proxyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Uri>,
                1usize,
            >("ProxyUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProxyUri", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = unsafe {
            method.invoke_unchecked((), (proxyName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("System.Runtime.Serialization.ISerializable.GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.Runtime.Serialization.ISerializable.GetObjectData",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeUpdateFromRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UnsafeUpdateFromRegistry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnsafeUpdateFromRegistry", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRegExList(
        &mut self,
        canThrow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateRegExList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateRegExList", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (canThrow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializationInfo, streamingContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri__cordl_bool_Il2CppArray_ICredentials1(
        &mut self,
        Address: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        BypassOnLocal: bool,
        BypassList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        Credentials: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    bool,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (Address, BypassOnLocal, BypassList, Credentials),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        enableAutoproxy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (enableAutoproxy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials>,
                0usize,
            >("get_Credentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Credentials", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ICredentials> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ScriptEngine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::AutoWebProxyScriptEngine>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Net::AutoWebProxyScriptEngine>,
                0usize,
            >("get_ScriptEngine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ScriptEngine", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::AutoWebProxyScriptEngine,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_UseDefaultCredentials(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_UseDefaultCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_UseDefaultCredentials", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_UseDefaultCredentials(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_UseDefaultCredentials")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_UseDefaultCredentials", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl AsRef<crate::System::Net::IWebProxy> for crate::System::Net::WebProxy {
    fn as_ref(&self) -> &crate::System::Net::IWebProxy {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl AsMut<crate::System::Net::IWebProxy> for crate::System::Net::WebProxy {
    fn as_mut(&mut self) -> &mut crate::System::Net::IWebProxy {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Net::WebProxy {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+WebProxy")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Net::WebProxy {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}

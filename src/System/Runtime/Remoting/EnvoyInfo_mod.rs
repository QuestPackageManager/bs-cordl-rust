#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvoyInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub envoySinks: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Messaging::IMessageSink,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Runtime::Remoting::EnvoyInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "EnvoyInfo";
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
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl std::ops::Deref for crate::System::Runtime::Remoting::EnvoyInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::EnvoyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl crate::System::Runtime::Remoting::EnvoyInfo {
    pub fn New(
        sinks: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sinks))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sinks: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessageSink,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sinks))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_EnvoySinks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::IMessageSink,
                >,
                0usize,
            >("get_EnvoySinks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_EnvoySinks", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessageSink,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::Remoting::EnvoyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl AsRef<crate::System::Runtime::Remoting::IEnvoyInfo>
for crate::System::Runtime::Remoting::EnvoyInfo {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::IEnvoyInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+EnvoyInfo")]
impl AsMut<crate::System::Runtime::Remoting::IEnvoyInfo>
for crate::System::Runtime::Remoting::EnvoyInfo {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Remoting::IEnvoyInfo {
        unsafe { std::mem::transmute(self) }
    }
}

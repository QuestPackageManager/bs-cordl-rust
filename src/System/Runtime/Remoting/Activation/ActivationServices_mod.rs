#[cfg(feature = "cordl_class_System+Runtime+Remoting+Activation+ActivationServices")]
#[repr(C)]
#[derive(Debug)]
pub struct ActivationServices {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+Remoting+Activation+ActivationServices")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Activation";
    const CLASS_NAME: &'static str = "ActivationServices";
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
#[cfg(feature = "cordl_class_System+Runtime+Remoting+Activation+ActivationServices")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Remoting+Activation+ActivationServices")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+ActivationServices")]
impl crate::System::Runtime::Remoting::Activation::ActivationServices {
    pub fn Activate(
        proxy: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Proxies::RemotingProxy,
        >,
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Proxies::RemotingProxy,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Messaging::ConstructionCall,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Messaging::IMessage,
                        >,
                        2usize,
                    >("Activate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Activate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = unsafe { cordl_method_info.invoke_unchecked((), (proxy, ctorCall))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateUninitializedClassInstance(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("AllocateUninitializedClassInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateUninitializedClassInstance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateConstructionCall(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        activationAttributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
                        >,
                        3usize,
                    >("CreateConstructionCall")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateConstructionCall", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::ConstructionCall,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (_cordl_type, activationUrl, activationAttributes),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstanceFromMessage(
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Messaging::IMessage,
                        >,
                        1usize,
                    >("CreateInstanceFromMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateInstanceFromMessage", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ctorCall))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateProxyForType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("CreateProxyForType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateProxyForType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableProxyActivation(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EnableProxyActivation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableProxyActivation", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (_cordl_type, enable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoteActivate(
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Runtime::Remoting::Messaging::IMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Messaging::IMessage,
                        >,
                        1usize,
                    >("RemoteActivate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoteActivate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        > = unsafe { cordl_method_info.invoke_unchecked((), (ctorCall))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ConstructionActivator() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IActivator,
                        >,
                        0usize,
                    >("get_ConstructionActivator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ConstructionActivator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Remoting+Activation+ActivationServices")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::ActivationServices {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

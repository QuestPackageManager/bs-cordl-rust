#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct AppDomainLevelActivator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Activation::IActivator,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Activation";
    const CLASS_NAME: &'static str = "AppDomainLevelActivator";
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
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    pub fn Activate(
        &mut self,
        ctorCall: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
                        >,
                        1usize,
                    >("Activate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Activate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionReturnMessage,
        > = unsafe { method.invoke_unchecked(self, (ctorCall))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (activationUrl, next))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        activationUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        next: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Activation::IActivator,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (activationUrl, next))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_NextActivator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IActivator,
                        >,
                        0usize,
                    >("get_NextActivator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_NextActivator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IActivator,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl AsRef<crate::System::Runtime::Remoting::Activation::IActivator>
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Activation::IActivator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+AppDomainLevelActivator")]
impl AsMut<crate::System::Runtime::Remoting::Activation::IActivator>
for crate::System::Runtime::Remoting::Activation::AppDomainLevelActivator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Activation::IActivator {
        unsafe { std::mem::transmute(self) }
    }
}

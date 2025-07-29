#[cfg(
    feature = "cordl_class_System+Runtime+Remoting+Activation+RemoteActivationAttribute"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteActivationAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _contextProperties: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(
    feature = "cordl_class_System+Runtime+Remoting+Activation+RemoteActivationAttribute"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Activation";
    const CLASS_NAME: &'static str = "RemoteActivationAttribute";
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
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    pub fn GetPropertiesForNewContext(
        &mut self,
        ctor: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetPropertiesForNewContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPropertiesForNewContext", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsContextOK(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::Context,
        >,
        ctor: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Contexts::Context,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Activation::IConstructionCallMessage,
                            >,
                        ),
                        bool,
                        2usize,
                    >("IsContextOK")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsContextOK", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (ctx, ctor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        contextProperties: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contextProperties))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        contextProperties: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Collections::IList>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (contextProperties))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_System+Runtime+Remoting+Activation+RemoteActivationAttribute"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl AsRef<crate::System::Runtime::Remoting::Contexts::IContextAttribute>
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn as_ref(&self) -> &crate::System::Runtime::Remoting::Contexts::IContextAttribute {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Activation+RemoteActivationAttribute")]
impl AsMut<crate::System::Runtime::Remoting::Contexts::IContextAttribute>
for crate::System::Runtime::Remoting::Activation::RemoteActivationAttribute {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Remoting::Contexts::IContextAttribute {
        unsafe { std::mem::transmute(self) }
    }
}

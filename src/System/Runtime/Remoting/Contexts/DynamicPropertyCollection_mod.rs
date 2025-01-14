#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicPropertyCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _properties: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Contexts";
    const CLASS_NAME: &'static str = "DynamicPropertyCollection";
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
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
impl crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    #[cfg(
        feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
    )]
    pub type DynamicPropertyReg = crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg;
    pub fn FindProperty(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                i32,
                1usize,
            >("FindProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindProperty", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyMessage(
        &mut self,
        start: bool,
        msg: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Messaging::IMessage,
        >,
        client_site: bool,
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Remoting::Messaging::IMessage,
                    >,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("NotifyMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotifyMessage", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (start, msg, client_site, _cordl_async))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDynamicProperty(
        &mut self,
        prop: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IDynamicProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Contexts::IDynamicProperty,
                >),
                bool,
                1usize,
            >("RegisterDynamicProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterDynamicProperty", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (prop)) };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDynamicProperty(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("UnregisterDynamicProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterDynamicProperty", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
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
    pub fn get_HasProperties(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_HasProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_HasProperties", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicPropertyCollection_DynamicPropertyReg {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Property: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::IDynamicProperty,
    >,
    pub Sink: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Remoting::Contexts::IDynamicMessageSink,
    >,
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Contexts";
    const CLASS_NAME: &'static str = "DynamicPropertyCollection/DynamicPropertyReg";
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
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
impl crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
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
}
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

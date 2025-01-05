#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicPropertyCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _properties: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
}
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::DynamicPropertyCollection =>
    "System.Runtime.Remoting.Contexts"."DynamicPropertyCollection"
);
#[cfg(feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindProperty", (name))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyMessage", (start, msg, client_site, _cordl_async))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDynamicProperty(
        &mut self,
        prop: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Remoting::Contexts::IDynamicProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RegisterDynamicProperty", (prop))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDynamicProperty(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnregisterDynamicProperty", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasProperties(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasProperties", ())?;
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg
    => "System.Runtime.Remoting.Contexts"."DynamicPropertyCollection/DynamicPropertyReg"
);
#[cfg(
    feature = "System+Runtime+Remoting+Contexts+DynamicPropertyCollection+DynamicPropertyReg"
)]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Contexts::DynamicPropertyCollection_DynamicPropertyReg {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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

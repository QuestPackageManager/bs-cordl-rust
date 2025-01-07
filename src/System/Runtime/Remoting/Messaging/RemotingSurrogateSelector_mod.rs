#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingSurrogateSelector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISurrogateSelector,
    >,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting.Messaging";
    const CLASS_NAME: &'static str = "RemotingSurrogateSelector";
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl std::ops::Deref
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    pub fn GetSurrogate(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        context: crate::System::Runtime::Serialization::StreamingContext,
        ssout: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Serialization::ISurrogateSelector,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializationSurrogate,
        > = __cordl_object.invoke("GetSurrogate", (_cordl_type, context, ssout))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl AsRef<crate::System::Runtime::Serialization::ISurrogateSelector>
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISurrogateSelector {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+RemotingSurrogateSelector")]
impl AsMut<crate::System::Runtime::Serialization::ISurrogateSelector>
for crate::System::Runtime::Remoting::Messaging::RemotingSurrogateSelector {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::ISurrogateSelector {
        unsafe { std::mem::transmute(self) }
    }
}

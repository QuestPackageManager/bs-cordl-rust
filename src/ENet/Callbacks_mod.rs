#[cfg(feature = "ENet+Callbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct Callbacks {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub nativeCallbacks: crate::ENet::ENetCallbacks,
}
#[cfg(feature = "ENet+Callbacks")]
unsafe impl quest_hook::libil2cpp::Type for crate::ENet::Callbacks {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "ENet";
    const CLASS_NAME: &'static str = "Callbacks";
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
#[cfg(feature = "ENet+Callbacks")]
impl std::ops::Deref for crate::ENet::Callbacks {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Callbacks")]
impl std::ops::DerefMut for crate::ENet::Callbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Callbacks")]
impl crate::ENet::Callbacks {
    pub fn New(
        allocCallback: quest_hook::libil2cpp::Gc<crate::ENet::AllocCallback>,
        freeCallback: quest_hook::libil2cpp::Gc<crate::ENet::FreeCallback>,
        noMemoryCallback: quest_hook::libil2cpp::Gc<crate::ENet::NoMemoryCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (allocCallback, freeCallback, noMemoryCallback))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        allocCallback: quest_hook::libil2cpp::Gc<crate::ENet::AllocCallback>,
        freeCallback: quest_hook::libil2cpp::Gc<crate::ENet::FreeCallback>,
        noMemoryCallback: quest_hook::libil2cpp::Gc<crate::ENet::NoMemoryCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (allocCallback, freeCallback, noMemoryCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::ENetCallbacks> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::ENet::ENetCallbacks = __cordl_object
            .invoke("get_NativeData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_NativeData(
        &mut self,
        value: crate::ENet::ENetCallbacks,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NativeData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+Callbacks")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Callbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

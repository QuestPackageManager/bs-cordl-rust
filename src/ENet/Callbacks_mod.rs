#[cfg(feature = "ENet+Callbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct Callbacks {
    __cordl_parent: crate::System::Object,
    pub nativeCallbacks: crate::ENet::ENetCallbacks,
}
#[cfg(feature = "ENet+Callbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::Callbacks => "ENet"."Callbacks"
);
#[cfg(feature = "ENet+Callbacks")]
impl std::ops::Deref for crate::ENet::Callbacks {
    type Target = crate::System::Object;
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
        allocCallback: *mut crate::ENet::AllocCallback,
        freeCallback: *mut crate::ENet::FreeCallback,
        noMemoryCallback: *mut crate::ENet::NoMemoryCallback,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (allocCallback, freeCallback, noMemoryCallback))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        allocCallback: *mut crate::ENet::AllocCallback,
        freeCallback: *mut crate::ENet::FreeCallback,
        noMemoryCallback: *mut crate::ENet::NoMemoryCallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (allocCallback, freeCallback, noMemoryCallback))?;
        Ok(__cordl_ret)
    }
    pub fn get_NativeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::ENetCallbacks> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::ENet::ENetCallbacks = __cordl_object
            .invoke("get_NativeData", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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

#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoPInvokeCallbackAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::AOT::MonoPInvokeCallbackAttribute => "AOT"
    ."MonoPInvokeCallbackAttribute"
);
#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
impl std::ops::Deref for crate::AOT::MonoPInvokeCallbackAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
impl std::ops::DerefMut for crate::AOT::MonoPInvokeCallbackAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
impl crate::AOT::MonoPInvokeCallbackAttribute {
    pub fn New(
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AOT+MonoPInvokeCallbackAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::AOT::MonoPInvokeCallbackAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

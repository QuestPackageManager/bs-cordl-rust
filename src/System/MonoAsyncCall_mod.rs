#[cfg(feature = "System+MonoAsyncCall")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoAsyncCall {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub msg: *mut quest_hook::libil2cpp::Il2CppObject,
    pub cb_method: crate::System::IntPtr,
    pub cb_target: *mut quest_hook::libil2cpp::Il2CppObject,
    pub state: *mut quest_hook::libil2cpp::Il2CppObject,
    pub res: *mut quest_hook::libil2cpp::Il2CppObject,
    pub out_args: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+MonoAsyncCall")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MonoAsyncCall => "System"
    ."MonoAsyncCall"
);
#[cfg(feature = "System+MonoAsyncCall")]
impl std::ops::Deref for crate::System::MonoAsyncCall {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoAsyncCall")]
impl std::ops::DerefMut for crate::System::MonoAsyncCall {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MonoAsyncCall")]
impl crate::System::MonoAsyncCall {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+MonoAsyncCall")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MonoAsyncCall {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

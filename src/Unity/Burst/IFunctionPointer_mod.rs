#[cfg(feature = "Unity+Burst+IFunctionPointer")]
#[repr(C)]
#[derive(Debug)]
pub struct IFunctionPointer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+IFunctionPointer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::IFunctionPointer => "Unity.Burst"
    ."IFunctionPointer"
);
#[cfg(feature = "Unity+Burst+IFunctionPointer")]
impl std::ops::Deref for crate::Unity::Burst::IFunctionPointer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+IFunctionPointer")]
impl std::ops::DerefMut for crate::Unity::Burst::IFunctionPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+IFunctionPointer")]
impl crate::Unity::Burst::IFunctionPointer {
    pub fn FromIntPtr(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Unity::Burst::IFunctionPointer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Unity::Burst::IFunctionPointer = __cordl_object
            .invoke("FromIntPtr", (ptr))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Unity+Burst+IFunctionPointer")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::IFunctionPointer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

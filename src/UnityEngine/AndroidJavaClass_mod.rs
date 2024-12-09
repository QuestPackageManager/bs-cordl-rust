#[cfg(feature = "UnityEngine+AndroidJavaClass")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJavaClass {
    __cordl_parent: crate::UnityEngine::AndroidJavaObject,
}
#[cfg(feature = "UnityEngine+AndroidJavaClass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJavaClass => "UnityEngine"
    ."AndroidJavaClass"
);
#[cfg(feature = "UnityEngine+AndroidJavaClass")]
impl std::ops::Deref for crate::UnityEngine::AndroidJavaClass {
    type Target = crate::UnityEngine::AndroidJavaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaClass")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJavaClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaClass")]
impl crate::UnityEngine::AndroidJavaClass {
    pub fn New_Il2CppString0(
        className: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (className))?;
        Ok(__cordl_object)
    }
    pub fn New_IntPtr1(
        jclass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (jclass))?;
        Ok(__cordl_object)
    }
    pub fn _AndroidJavaClass(
        &mut self,
        className: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("_AndroidJavaClass", (className))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        className: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (className))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IntPtr1(
        &mut self,
        jclass: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (jclass))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaClass")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJavaClass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

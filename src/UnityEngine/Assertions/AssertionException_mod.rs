#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertionException {
    __cordl_parent: crate::System::Exception,
    pub m_UserMessage: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Assertions::AssertionException =>
    "UnityEngine.Assertions"."AssertionException"
);
#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
impl std::ops::Deref for crate::UnityEngine::Assertions::AssertionException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
impl std::ops::DerefMut for crate::UnityEngine::Assertions::AssertionException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
impl crate::UnityEngine::Assertions::AssertionException {
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Message", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        message: *mut crate::System::String,
        userMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, userMessage))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        message: *mut crate::System::String,
        userMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, userMessage))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionException")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Assertions::AssertionException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

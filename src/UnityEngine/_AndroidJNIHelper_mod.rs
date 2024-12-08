#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct _AndroidJNIHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::_AndroidJNIHelper => "UnityEngine"
    ."_AndroidJNIHelper"
);
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl std::ops::Deref for crate::UnityEngine::_AndroidJNIHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl std::ops::DerefMut for crate::UnityEngine::_AndroidJNIHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl crate::UnityEngine::_AndroidJNIHelper {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+_AndroidJNIHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::_AndroidJNIHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

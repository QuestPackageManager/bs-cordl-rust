#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct ClassLibraryInitializer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ClassLibraryInitializer =>
    "UnityEngine"."ClassLibraryInitializer"
);
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl std::ops::Deref for crate::UnityEngine::ClassLibraryInitializer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl std::ops::DerefMut for crate::UnityEngine::ClassLibraryInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl crate::UnityEngine::ClassLibraryInitializer {
    pub fn Init() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAssemblyRedirections() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitAssemblyRedirections", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitStdErrWithHandle(
        fileHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitStdErrWithHandle", (fileHandle))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ClassLibraryInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

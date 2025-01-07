#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct ClassLibraryInitializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ClassLibraryInitializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ClassLibraryInitializer";
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
#[cfg(feature = "UnityEngine+ClassLibraryInitializer")]
impl std::ops::Deref for crate::UnityEngine::ClassLibraryInitializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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

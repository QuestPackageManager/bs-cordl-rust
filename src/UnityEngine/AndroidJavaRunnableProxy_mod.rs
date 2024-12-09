#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJavaRunnableProxy {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub mRunnable: *mut crate::UnityEngine::AndroidJavaRunnable,
}
#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJavaRunnableProxy =>
    "UnityEngine"."AndroidJavaRunnableProxy"
);
#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
impl std::ops::Deref for crate::UnityEngine::AndroidJavaRunnableProxy {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJavaRunnableProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
impl crate::UnityEngine::AndroidJavaRunnableProxy {
    pub fn Invoke(
        &mut self,
        methodName: *mut quest_hook::libil2cpp::Il2CppString,
        javaArgs: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (methodName, javaArgs))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        runnable: *mut crate::UnityEngine::AndroidJavaRunnable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (runnable))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        runnable: *mut crate::UnityEngine::AndroidJavaRunnable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (runnable))?;
        Ok(__cordl_ret)
    }
    pub fn run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("run", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AndroidJavaRunnableProxy")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJavaRunnableProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

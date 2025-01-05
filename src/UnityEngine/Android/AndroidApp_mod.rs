#[cfg(feature = "UnityEngine+Android+AndroidApp")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidApp {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidApp =>
    "UnityEngine.Android"."AndroidApp"
);
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidApp {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl crate::UnityEngine::Android::AndroidApp {
    pub fn AcquireContextAndActivity() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireContextAndActivity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Activity() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AndroidJavaObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Activity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Context() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AndroidJavaObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Context", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UnityPlayerRaw() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_UnityPlayerRaw", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Android::AndroidApp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

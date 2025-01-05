#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
#[repr(C)]
#[derive(Debug)]
pub struct SplashScreen {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::SplashScreen =>
    "UnityEngine.Rendering"."SplashScreen"
);
#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
impl std::ops::Deref for crate::UnityEngine::Rendering::SplashScreen {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::SplashScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
impl crate::UnityEngine::Rendering::SplashScreen {
    pub fn get_isFinished() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isFinished", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+SplashScreen")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::SplashScreen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

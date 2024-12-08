#[cfg(feature = "FadeInOnSceneTransitionFinished")]
#[repr(C)]
#[derive(Debug)]
pub struct FadeInOnSceneTransitionFinished {
    __cordl_parent: ZenjectSafeBehaviour,
    pub _fadeInOut: *mut FadeInOutController,
    pub _gameScenesManager: *mut GameScenesManager,
}
#[cfg(feature = "FadeInOnSceneTransitionFinished")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FadeInOnSceneTransitionFinished => ""
    ."FadeInOnSceneTransitionFinished"
);
#[cfg(feature = "FadeInOnSceneTransitionFinished")]
impl std::ops::Deref for FadeInOnSceneTransitionFinished {
    type Target = ZenjectSafeBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FadeInOnSceneTransitionFinished")]
impl std::ops::DerefMut for FadeInOnSceneTransitionFinished {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FadeInOnSceneTransitionFinished")]
impl FadeInOnSceneTransitionFinished {
    #[cfg(feature = "FadeInOnSceneTransitionFinished+_FadeCoroutine_d__4")]
    pub type _FadeCoroutine_d__4 = crate::GlobalNamespace::FadeInOnSceneTransitionFinished__FadeCoroutine_d__4;
    pub fn FadeCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("FadeCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnablePostInjection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnablePostInjection", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "FadeInOnSceneTransitionFinished")]
impl quest_hook::libil2cpp::ObjectType for FadeInOnSceneTransitionFinished {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
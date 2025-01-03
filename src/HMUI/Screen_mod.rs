#[cfg(feature = "HMUI+Screen")]
#[repr(C)]
#[derive(Debug)]
pub struct Screen {
    __cordl_parent: crate::HMUI::ScreenBase,
    pub _rootViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    pub _isBeingDestroyed: bool,
}
#[cfg(feature = "HMUI+Screen")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::Screen => "HMUI"."Screen"
);
#[cfg(feature = "HMUI+Screen")]
impl std::ops::Deref for crate::HMUI::Screen {
    type Target = crate::HMUI::ScreenBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+Screen")]
impl std::ops::DerefMut for crate::HMUI::Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+Screen")]
impl crate::HMUI::Screen {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRootViewController(
        &mut self,
        newRootViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRootViewController", (newRootViewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionCoroutine(
        &mut self,
        newRootViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke("TransitionCoroutine", (newRootViewController, animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isBeingDestroyed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isBeingDestroyed", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+Screen")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::Screen {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

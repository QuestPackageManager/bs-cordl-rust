#[cfg(feature = "OVRPlatformMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPlatformMenu {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub inputCode: crate::GlobalNamespace::OVRInput_RawButton,
    pub shortPressHandler: crate::GlobalNamespace::OVRPlatformMenu_eHandler,
    pub OnShortPress: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
}
#[cfg(feature = "OVRPlatformMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlatformMenu => ""
    ."OVRPlatformMenu"
);
#[cfg(feature = "OVRPlatformMenu")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPlatformMenu {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlatformMenu")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPlatformMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPlatformMenu")]
impl crate::GlobalNamespace::OVRPlatformMenu {
    #[cfg(feature = "OVRPlatformMenu+eBackButtonAction")]
    pub type eBackButtonAction = crate::GlobalNamespace::OVRPlatformMenu_eBackButtonAction;
    #[cfg(feature = "OVRPlatformMenu+eHandler")]
    pub type eHandler = crate::GlobalNamespace::OVRPlatformMenu_eHandler;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBackButtonState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlatformMenu_eBackButtonAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlatformMenu_eBackButtonAction = __cordl_object
            .invoke("HandleBackButtonState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RetreatOneLevel() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RetreatOneLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowConfirmQuitMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowConfirmQuitMenu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "OVRPlatformMenu")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRPlatformMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPlatformMenu+eBackButtonAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlatformMenu_eBackButtonAction {
    #[default]
    NONE = 0i32,
    SHORT_PRESS = 1i32,
}
#[cfg(feature = "OVRPlatformMenu+eBackButtonAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPlatformMenu_eBackButtonAction => ""
    ."OVRPlatformMenu/eBackButtonAction"
);
#[cfg(feature = "OVRPlatformMenu+eHandler")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPlatformMenu_eHandler {
    #[default]
    RetreatOneLevel = 1i32,
    ShowConfirmQuit = 0i32,
}
#[cfg(feature = "OVRPlatformMenu+eHandler")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPlatformMenu_eHandler => ""
    ."OVRPlatformMenu/eHandler"
);

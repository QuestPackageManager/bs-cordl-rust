#[cfg(feature = "StandardLevelBuyInfoView")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelBuyInfoView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _text: *mut crate::TMPro::TextMeshProUGUI,
    pub _buyLevelButton: *mut crate::UnityEngine::UI::Button,
    pub _openPackButton: *mut crate::UnityEngine::UI::Button,
    pub _buyPackButton: *mut crate::UnityEngine::UI::Button,
    pub _steamMessageGameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "StandardLevelBuyInfoView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardLevelBuyInfoView => ""
    ."StandardLevelBuyInfoView"
);
#[cfg(feature = "StandardLevelBuyInfoView")]
impl std::ops::Deref for StandardLevelBuyInfoView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelBuyInfoView")]
impl std::ops::DerefMut for StandardLevelBuyInfoView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelBuyInfoView")]
impl StandardLevelBuyInfoView {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshView(
        &mut self,
        infoText: *mut crate::System::String,
        canBuyPack: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshView", (infoText, canBuyPack))?;
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
    pub fn get_buyLevelButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_buyLevelButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_buyPackButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_buyPackButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_openPackButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_openPackButton", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelBuyInfoView")]
impl quest_hook::libil2cpp::ObjectType for StandardLevelBuyInfoView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
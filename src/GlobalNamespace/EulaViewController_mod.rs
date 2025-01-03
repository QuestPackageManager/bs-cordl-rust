#[cfg(feature = "EulaViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct EulaViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _agreeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _doNotAgreeButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _continueButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _disabledButtonDelay: f32,
    pub _disableButtonsProgress: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub _textPageScrollView: quest_hook::libil2cpp::Gc<crate::HMUI::TextPageScrollView>,
    pub _eulaLocalizedTextAsset: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalizedTextAsset,
    >,
    pub _updateNoticeLocalizationKey: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EulaViewController_InitData,
    >,
    pub _coroutineStarter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ICoroutineStarter,
    >,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::EulaViewController_ButtonType>,
    >,
    pub _showUpdate: bool,
    pub _showOnlyContinueButton: bool,
    pub _buttonsCoroutine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
}
#[cfg(feature = "EulaViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EulaViewController => ""
    ."EulaViewController"
);
#[cfg(feature = "EulaViewController")]
impl std::ops::Deref for crate::GlobalNamespace::EulaViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EulaViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::EulaViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EulaViewController")]
impl crate::GlobalNamespace::EulaViewController {
    #[cfg(feature = "EulaViewController+ButtonType")]
    pub type ButtonType = crate::GlobalNamespace::EulaViewController_ButtonType;
    #[cfg(feature = "EulaViewController+InitData")]
    pub type InitData = crate::GlobalNamespace::EulaViewController_InitData;
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
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableButtonsCoroutine(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("EnableButtonsCoroutine", (delay))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        showUpdate: bool,
        showOnlyContinueButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (showUpdate, showOnlyContinueButton))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _DidActivate_b__19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__19_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__19_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_2", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::EulaViewController_ButtonType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::GlobalNamespace::EulaViewController_ButtonType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EulaViewController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::EulaViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EulaViewController+ButtonType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EulaViewController_ButtonType {
    Agree = 0i32,
    DoNotAgree = 1i32,
}
#[cfg(feature = "EulaViewController+ButtonType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EulaViewController_ButtonType
    => ""."EulaViewController/ButtonType"
);
#[cfg(feature = "EulaViewController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct EulaViewController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub showDoNotAgreeButton: bool,
}
#[cfg(feature = "EulaViewController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EulaViewController_InitData =>
    ""."EulaViewController/InitData"
);
#[cfg(feature = "EulaViewController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::EulaViewController_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EulaViewController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::EulaViewController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EulaViewController+InitData")]
impl crate::GlobalNamespace::EulaViewController_InitData {
    pub fn New(
        showDoNotAgreeButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (showDoNotAgreeButton))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        showDoNotAgreeButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (showDoNotAgreeButton))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EulaViewController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EulaViewController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

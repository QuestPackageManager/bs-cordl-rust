#[cfg(feature = "EnterPlayerGuestNameViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct EnterPlayerGuestNameViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _playerNameWasEnteredSignal: *mut StringSignal,
    pub _uiKeyboard: *mut crate::HMUI::UIKeyboard,
    pub _nameInputFieldView: *mut crate::HMUI::InputFieldView,
    pub _guestNameButtonsListItemsList: *mut GuestNameButtonsListItemsList,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _didFinishCallback: *mut crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate,
}
#[cfg(feature = "EnterPlayerGuestNameViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnterPlayerGuestNameViewController => ""
    ."EnterPlayerGuestNameViewController"
);
#[cfg(feature = "EnterPlayerGuestNameViewController")]
impl std::ops::Deref for EnterPlayerGuestNameViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController")]
impl std::ops::DerefMut for EnterPlayerGuestNameViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController")]
impl EnterPlayerGuestNameViewController {
    pub const kMaxPlayerNameCompoundLength: i32 = 40i32;
    pub const kMaxShowPlayer: i32 = 5i32;
    #[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
    pub type FinishDelegate = crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate;
    #[cfg(feature = "EnterPlayerGuestNameViewController+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::GlobalNamespace::EnterPlayerGuestNameViewController___c__DisplayClass10_0;
    #[cfg(feature = "EnterPlayerGuestNameViewController+__c__DisplayClass10_1")]
    pub type __c__DisplayClass10_1 = crate::GlobalNamespace::EnterPlayerGuestNameViewController___c__DisplayClass10_1;
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
        Ok(__cordl_ret)
    }
    pub fn OkButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OkButtonPressed", ())?;
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
    pub fn Init(
        &mut self,
        didFinishCallback: *mut crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (didFinishCallback))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController")]
impl quest_hook::libil2cpp::ObjectType for EnterPlayerGuestNameViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct EnterPlayerGuestNameViewController_FinishDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate => ""
    ."EnterPlayerGuestNameViewController/FinishDelegate"
);
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
impl crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate {
    pub fn Invoke(
        &mut self,
        viewController: *mut EnterPlayerGuestNameViewController,
        playerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (viewController, playerName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn BeginInvoke(
        &mut self,
        viewController: *mut EnterPlayerGuestNameViewController,
        playerName: *mut crate::System::String,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (viewController, playerName, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EnterPlayerGuestNameViewController+FinishDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnterPlayerGuestNameViewController_FinishDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

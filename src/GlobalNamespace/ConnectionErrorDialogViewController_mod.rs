#[cfg(feature = "ConnectionErrorDialogViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionErrorDialogViewController {
    __cordl_parent: crate::GlobalNamespace::SimpleDialogPromptViewController,
}
#[cfg(feature = "ConnectionErrorDialogViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ConnectionErrorDialogViewController => ""
    ."ConnectionErrorDialogViewController"
);
#[cfg(feature = "ConnectionErrorDialogViewController")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectionErrorDialogViewController {
    type Target = crate::GlobalNamespace::SimpleDialogPromptViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionErrorDialogViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectionErrorDialogViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionErrorDialogViewController")]
impl crate::GlobalNamespace::ConnectionErrorDialogViewController {
    #[cfg(feature = "ConnectionErrorDialogViewController+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::ConnectionErrorDialogViewController___c__DisplayClass0_0;
    pub fn Init(
        &mut self,
        reason: crate::GlobalNamespace::DisconnectedReason,
        buttonAction: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (reason, buttonAction))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "ConnectionErrorDialogViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectionErrorDialogViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

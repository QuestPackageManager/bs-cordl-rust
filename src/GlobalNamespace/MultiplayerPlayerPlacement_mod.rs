#[cfg(feature = "MultiplayerPlayerPlacement")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayerPlacement {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerPlayerPlacement => ""
    ."MultiplayerPlayerPlacement"
);
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl std::ops::Deref for MultiplayerPlayerPlacement {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl std::ops::DerefMut for MultiplayerPlayerPlacement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl MultiplayerPlayerPlacement {
    #[cfg(feature = "MultiplayerPlayerPlacement+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerPlayerPlacement___c;
}
#[cfg(feature = "MultiplayerPlayerPlacement")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerPlayerPlacement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

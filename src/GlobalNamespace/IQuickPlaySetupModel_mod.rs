#[cfg(feature = "IQuickPlaySetupModel")]
#[repr(C)]
#[derive(Debug)]
pub struct IQuickPlaySetupModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IQuickPlaySetupModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IQuickPlaySetupModel => ""
    ."IQuickPlaySetupModel"
);
#[cfg(feature = "IQuickPlaySetupModel")]
impl std::ops::Deref for crate::GlobalNamespace::IQuickPlaySetupModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IQuickPlaySetupModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::IQuickPlaySetupModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IQuickPlaySetupModel")]
impl crate::GlobalNamespace::IQuickPlaySetupModel {
    pub fn GetQuickPlaySetupAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::QuickPlaySetupData>,
            >,
        > = __cordl_object.invoke("GetQuickPlaySetupAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IQuickPlaySetupModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IQuickPlaySetupModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

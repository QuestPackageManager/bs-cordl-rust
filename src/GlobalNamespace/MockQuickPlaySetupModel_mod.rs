#[cfg(feature = "MockQuickPlaySetupModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockQuickPlaySetupModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _quickPlaySetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::QuickPlaySetupData,
    >,
}
#[cfg(feature = "MockQuickPlaySetupModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockQuickPlaySetupModel => ""
    ."MockQuickPlaySetupModel"
);
#[cfg(feature = "MockQuickPlaySetupModel")]
impl std::ops::Deref for crate::GlobalNamespace::MockQuickPlaySetupModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockQuickPlaySetupModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockQuickPlaySetupModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockQuickPlaySetupModel")]
impl crate::GlobalNamespace::MockQuickPlaySetupModel {
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "MockQuickPlaySetupModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockQuickPlaySetupModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockQuickPlaySetupModel")]
impl AsRef<crate::GlobalNamespace::IQuickPlaySetupModel>
for crate::GlobalNamespace::MockQuickPlaySetupModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IQuickPlaySetupModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockQuickPlaySetupModel")]
impl AsMut<crate::GlobalNamespace::IQuickPlaySetupModel>
for crate::GlobalNamespace::MockQuickPlaySetupModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IQuickPlaySetupModel {
        unsafe { std::mem::transmute(self) }
    }
}

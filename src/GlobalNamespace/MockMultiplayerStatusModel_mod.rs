#[cfg(feature = "MockMultiplayerStatusModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockMultiplayerStatusModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerStatusData: *mut crate::GlobalNamespace::MultiplayerStatusData,
}
#[cfg(feature = "MockMultiplayerStatusModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockMultiplayerStatusModel =>
    ""."MockMultiplayerStatusModel"
);
#[cfg(feature = "MockMultiplayerStatusModel")]
impl std::ops::Deref for crate::GlobalNamespace::MockMultiplayerStatusModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockMultiplayerStatusModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl crate::GlobalNamespace::MockMultiplayerStatusModel {
    pub fn GetMultiplayerStatusAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MultiplayerStatusData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::MultiplayerStatusData,
            >,
        > = __cordl_object.invoke("GetMultiplayerStatusAsync", (cancellationToken))?;
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
#[cfg(feature = "MockMultiplayerStatusModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockMultiplayerStatusModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl AsRef<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MockMultiplayerStatusModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl AsMut<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MockMultiplayerStatusModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}

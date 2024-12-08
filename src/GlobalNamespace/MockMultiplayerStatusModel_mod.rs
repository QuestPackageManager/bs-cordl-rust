#[cfg(feature = "MockMultiplayerStatusModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockMultiplayerStatusModel {
    __cordl_parent: crate::System::Object,
    pub _multiplayerStatusData: *mut MultiplayerStatusData,
}
#[cfg(feature = "MockMultiplayerStatusModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockMultiplayerStatusModel => ""
    ."MockMultiplayerStatusModel"
);
#[cfg(feature = "MockMultiplayerStatusModel")]
impl std::ops::Deref for MockMultiplayerStatusModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl std::ops::DerefMut for MockMultiplayerStatusModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl MockMultiplayerStatusModel {
    pub fn GetMultiplayerStatusAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut MultiplayerStatusData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut MultiplayerStatusData,
        > = __cordl_object.invoke("GetMultiplayerStatusAsync", (cancellationToken))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockMultiplayerStatusModel")]
impl quest_hook::libil2cpp::ObjectType for MockMultiplayerStatusModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

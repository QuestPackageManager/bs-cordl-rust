#[cfg(feature = "MultiplayerStatusModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkConfig,
    >,
    pub _client: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpClient>,
    pub _request: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            *mut crate::GlobalNamespace::MultiplayerStatusData,
        >,
    >,
}
#[cfg(feature = "MultiplayerStatusModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerStatusModel => ""
    ."MultiplayerStatusModel"
);
#[cfg(feature = "MultiplayerStatusModel")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerStatusModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerStatusModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl crate::GlobalNamespace::MultiplayerStatusModel {
    pub const kRequestTimeoutSeconds: i32 = 60i32;
    #[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
    pub type MultiplayerStatusDataFB = crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB;
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
    pub fn GetMultiplayerStatusAsyncInternal(
        &mut self,
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
        > = __cordl_object.invoke("GetMultiplayerStatusAsyncInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAvailabilityTaskValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAvailabilityTaskValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartRequest", ())?;
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
#[cfg(feature = "MultiplayerStatusModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl AsRef<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerStatusModel")]
impl AsMut<crate::GlobalNamespace::IMultiplayerStatusModel>
for crate::GlobalNamespace::MultiplayerStatusModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerStatusModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerStatusModel_MultiplayerStatusDataFB {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub data: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MultiplayerStatusData,
        >,
    >,
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB => ""
    ."MultiplayerStatusModel/MultiplayerStatusDataFB"
);
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
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
#[cfg(feature = "MultiplayerStatusModel+MultiplayerStatusDataFB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerStatusModel_MultiplayerStatusDataFB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

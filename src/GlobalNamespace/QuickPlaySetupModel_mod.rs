#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupModel_QuickPlaySetupDataFB {
    __cordl_parent: crate::System::Object,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<*mut QuickPlaySetupData>,
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB => ""
    ."QuickPlaySetupModel/QuickPlaySetupDataFB"
);
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
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
#[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupModel {
    __cordl_parent: crate::System::Object,
    pub _networkConfig: *mut INetworkConfig,
    pub _client: *mut crate::System::Net::Http::HttpClient,
    pub _request: *mut crate::System::Threading::Tasks::Task_1<*mut QuickPlaySetupData>,
    pub _lastRequestTime: crate::System::DateTime,
}
#[cfg(feature = "QuickPlaySetupModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for QuickPlaySetupModel => ""."QuickPlaySetupModel"
);
#[cfg(feature = "QuickPlaySetupModel")]
impl std::ops::Deref for QuickPlaySetupModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl std::ops::DerefMut for QuickPlaySetupModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupModel")]
impl QuickPlaySetupModel {
    pub const kRequestCacheTimeoutMinutes: i32 = 5i32;
    pub const kRequestTimeoutSeconds: i32 = 60i32;
    #[cfg(feature = "QuickPlaySetupModel+_GetQuickPlaySetupInternal_d__10")]
    pub type _GetQuickPlaySetupInternal_d__10 = crate::GlobalNamespace::QuickPlaySetupModel__GetQuickPlaySetupInternal_d__10;
    #[cfg(feature = "QuickPlaySetupModel+QuickPlaySetupDataFB")]
    pub type QuickPlaySetupDataFB = crate::GlobalNamespace::QuickPlaySetupModel_QuickPlaySetupDataFB;
    pub fn GetQuickPlaySetupAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut QuickPlaySetupData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut QuickPlaySetupData,
        > = __cordl_object.invoke("GetQuickPlaySetupAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn GetQuickPlaySetupInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut QuickPlaySetupData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut QuickPlaySetupData,
        > = __cordl_object.invoke("GetQuickPlaySetupInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsQuickPlaySetupTaskValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsQuickPlaySetupTaskValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsUrlValid(
        &mut self,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsUrlValid", (url))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StartRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartRequest", ())?;
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
}
#[cfg(feature = "QuickPlaySetupModel")]
impl quest_hook::libil2cpp::ObjectType for QuickPlaySetupModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
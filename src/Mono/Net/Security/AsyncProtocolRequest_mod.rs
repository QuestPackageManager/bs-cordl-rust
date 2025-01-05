#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncProtocolRequest {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Parent_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Mono::Net::Security::MobileAuthenticatedStream,
    >,
    pub _RunSynchronously_k__BackingField: bool,
    pub _UserResult_k__BackingField: i32,
    pub Started: i32,
    pub RequestedSize: i32,
    pub WriteRequested: i32,
    pub locker: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::AsyncProtocolRequest =>
    "Mono.Net.Security"."AsyncProtocolRequest"
);
#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
impl std::ops::Deref for crate::Mono::Net::Security::AsyncProtocolRequest {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
impl std::ops::DerefMut for crate::Mono::Net::Security::AsyncProtocolRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
impl crate::Mono::Net::Security::AsyncProtocolRequest {
    pub fn InnerRead(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<crate::System::Nullable_1<i32>>,
        > = __cordl_object.invoke("InnerRead", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        sync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, sync))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessOperation(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ProcessOperation", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestRead(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestRead", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestWrite(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(
        &mut self,
        status: crate::Mono::Net::Security::AsyncOperationStatus,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Net::Security::AsyncOperationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Net::Security::AsyncOperationStatus = __cordl_object
            .invoke("Run", (status))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartOperation(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::Mono::Net::Security::AsyncProtocolResult,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::Mono::Net::Security::AsyncProtocolResult,
                >,
            >,
        > = __cordl_object.invoke("StartOperation", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        sync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent, sync))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Net::Security::MobileAuthenticatedStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        > = __cordl_object.invoke("get_Parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RunSynchronously(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RunSynchronously", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserResult(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_UserResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_UserResult(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UserResult", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+AsyncProtocolRequest")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::AsyncProtocolRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

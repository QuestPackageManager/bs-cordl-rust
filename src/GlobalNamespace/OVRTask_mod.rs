#[cfg(feature = "OVRTask")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTask {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTask")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRTask => ""."OVRTask"
);
#[cfg(feature = "OVRTask")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTask {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl crate::GlobalNamespace::OVRTask {
    pub fn Create<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromGuid<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromGuid", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromRequest<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromRequest", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResult<TResult>(
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromResult", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Get", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_Guid0<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExisting", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_u64_1<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExisting", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetId(value: u64) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_ret: crate::System::Guid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_Guid0<TResult>(
        id: crate::System::Guid,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetResult", (id, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_u64_1<TResult>(
        id: u64,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetResult", (id, result))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTask")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

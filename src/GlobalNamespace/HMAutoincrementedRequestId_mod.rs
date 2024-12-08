#[cfg(feature = "HMAutoincrementedRequestId")]
#[repr(C)]
#[derive(Debug)]
pub struct HMAutoincrementedRequestId {
    __cordl_parent: crate::System::Object,
    pub _requestId: u64,
}
#[cfg(feature = "HMAutoincrementedRequestId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for HMAutoincrementedRequestId => ""
    ."HMAutoincrementedRequestId"
);
#[cfg(feature = "HMAutoincrementedRequestId")]
impl std::ops::Deref for HMAutoincrementedRequestId {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMAutoincrementedRequestId")]
impl std::ops::DerefMut for HMAutoincrementedRequestId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMAutoincrementedRequestId")]
impl HMAutoincrementedRequestId {
    pub fn Equals_HMAutoincrementedRequestId0(
        &mut self,
        obj: *mut HMAutoincrementedRequestId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
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
    pub fn get_RequestId(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_RequestId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMAutoincrementedRequestId")]
impl quest_hook::libil2cpp::ObjectType for HMAutoincrementedRequestId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
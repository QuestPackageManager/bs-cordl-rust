#[cfg(feature = "HMAsyncRequest")]
#[repr(C)]
#[derive(Debug)]
pub struct HMAsyncRequest {
    __cordl_parent: crate::GlobalNamespace::HMAutoincrementedRequestId,
    pub _cancelled: bool,
    pub _cancelHander: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HMAsyncRequest_CancelHander,
    >,
}
#[cfg(feature = "HMAsyncRequest")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HMAsyncRequest => ""
    ."HMAsyncRequest"
);
#[cfg(feature = "HMAsyncRequest")]
impl std::ops::Deref for crate::GlobalNamespace::HMAsyncRequest {
    type Target = crate::GlobalNamespace::HMAutoincrementedRequestId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMAsyncRequest")]
impl std::ops::DerefMut for crate::GlobalNamespace::HMAsyncRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMAsyncRequest")]
impl crate::GlobalNamespace::HMAsyncRequest {
    #[cfg(feature = "HMAsyncRequest+CancelHander")]
    pub type CancelHander = crate::GlobalNamespace::HMAsyncRequest_CancelHander;
    pub fn Cancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cancel", ())?;
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
    pub fn get_CancelHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest_CancelHander>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest_CancelHander,
        > = __cordl_object.invoke("get_CancelHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cancelled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_cancelled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CancelHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest_CancelHander,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CancelHandler", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMAsyncRequest")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::HMAsyncRequest {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMAsyncRequest+CancelHander")]
#[repr(C)]
#[derive(Debug)]
pub struct HMAsyncRequest_CancelHander {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMAsyncRequest+CancelHander")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HMAsyncRequest_CancelHander =>
    ""."HMAsyncRequest/CancelHander"
);
#[cfg(feature = "HMAsyncRequest+CancelHander")]
impl std::ops::Deref for crate::GlobalNamespace::HMAsyncRequest_CancelHander {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMAsyncRequest+CancelHander")]
impl std::ops::DerefMut for crate::GlobalNamespace::HMAsyncRequest_CancelHander {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMAsyncRequest+CancelHander")]
impl crate::GlobalNamespace::HMAsyncRequest_CancelHander {
    pub fn BeginInvoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (request, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMAsyncRequest+CancelHander")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HMAsyncRequest_CancelHander {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

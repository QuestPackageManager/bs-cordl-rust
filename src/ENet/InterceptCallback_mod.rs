#[cfg(feature = "ENet+InterceptCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct InterceptCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "ENet+InterceptCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::InterceptCallback => "ENet"
    ."InterceptCallback"
);
#[cfg(feature = "ENet+InterceptCallback")]
impl std::ops::Deref for crate::ENet::InterceptCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+InterceptCallback")]
impl std::ops::DerefMut for crate::ENet::InterceptCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+InterceptCallback")]
impl crate::ENet::InterceptCallback {
    pub fn BeginInvoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        receivedData: crate::System::IntPtr,
        receivedDataLength: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (event, address, receivedData, receivedDataLength, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("EndInvoke", (event, address, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::Event>,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::Address>,
        receivedData: crate::System::IntPtr,
        receivedDataLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Invoke", (event, address, receivedData, receivedDataLength))?;
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
#[cfg(feature = "ENet+InterceptCallback")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::InterceptCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

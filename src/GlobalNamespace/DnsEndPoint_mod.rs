#[cfg(feature = "DnsEndPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct DnsEndPoint {
    __cordl_parent: crate::System::Object,
    pub hostName: *mut crate::System::String,
    pub port: i32,
    pub _getEndPointTask: *mut crate::System::Threading::Tasks::Task_1<
        *mut crate::System::Net::IPEndPoint,
    >,
}
#[cfg(feature = "DnsEndPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DnsEndPoint => ""."DnsEndPoint"
);
#[cfg(feature = "DnsEndPoint")]
impl std::ops::Deref for crate::GlobalNamespace::DnsEndPoint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DnsEndPoint")]
impl std::ops::DerefMut for crate::GlobalNamespace::DnsEndPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DnsEndPoint")]
impl crate::GlobalNamespace::DnsEndPoint {
    pub fn Equals_DnsEndPoint1(
        &mut self,
        other: *mut crate::GlobalNamespace::DnsEndPoint,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("GetEndPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEndPointAsync(
        &mut self,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::Net::IPEndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Net::IPEndPoint,
        > = __cordl_object.invoke("GetEndPointAsync", (taskUtility))?;
        Ok(__cordl_ret)
    }
    pub fn GetEndPointInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("GetEndPointInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_IPEndPoint1(
        endPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (endPoint))?;
        Ok(__cordl_object)
    }
    pub fn New_String_i32_0(
        hostName: *mut crate::System::String,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hostName, port))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IPEndPoint1(
        &mut self,
        endPoint: *mut crate::System::Net::IPEndPoint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (endPoint))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_i32_0(
        &mut self,
        hostName: *mut crate::System::String,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hostName, port))?;
        Ok(__cordl_ret)
    }
    pub fn get_endPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_endPoint", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "DnsEndPoint")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DnsEndPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

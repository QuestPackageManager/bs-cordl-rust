#[cfg(feature = "System+MulticastDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct MulticastDelegate {
    __cordl_parent: crate::System::Delegate,
    pub delegates: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Delegate>,
}
#[cfg(feature = "System+MulticastDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MulticastDelegate => "System"
    ."MulticastDelegate"
);
#[cfg(feature = "System+MulticastDelegate")]
impl std::ops::Deref for crate::System::MulticastDelegate {
    type Target = crate::System::Delegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MulticastDelegate")]
impl std::ops::DerefMut for crate::System::MulticastDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MulticastDelegate")]
impl crate::System::MulticastDelegate {
    pub fn CombineImpl(
        &mut self,
        follow: *mut crate::System::Delegate,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Delegate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Delegate = __cordl_object
            .invoke("CombineImpl", (follow))?;
        Ok(__cordl_ret)
    }
    pub fn DynamicInvokeImpl(
        &mut self,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("DynamicInvokeImpl", (args))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
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
    pub fn GetInvocationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Delegate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Delegate,
        > = __cordl_object.invoke("GetInvocationList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMethodImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::MethodInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::MethodInfo = __cordl_object
            .invoke("GetMethodImpl", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf(
        &mut self,
        haystack: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Delegate>,
        needle: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LastIndexOf", (haystack, needle))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveImpl(
        &mut self,
        value: *mut crate::System::Delegate,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Delegate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Delegate = __cordl_object
            .invoke("RemoveImpl", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+MulticastDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MulticastDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

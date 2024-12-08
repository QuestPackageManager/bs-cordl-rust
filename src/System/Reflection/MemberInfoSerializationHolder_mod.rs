#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberInfoSerializationHolder {
    __cordl_parent: crate::System::Object,
    pub m_memberName: *mut crate::System::String,
    pub m_reflectedType: *mut crate::System::RuntimeType,
    pub m_signature: *mut crate::System::String,
    pub m_signature2: *mut crate::System::String,
    pub m_memberType: crate::System::Reflection::MemberTypes,
    pub m_info: *mut crate::System::Runtime::Serialization::SerializationInfo,
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::MemberInfoSerializationHolder => "System.Reflection"
    ."MemberInfoSerializationHolder"
);
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl std::ops::Deref for crate::System::Reflection::MemberInfoSerializationHolder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl std::ops::DerefMut for crate::System::Reflection::MemberInfoSerializationHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl crate::System::Reflection::MemberInfoSerializationHolder {
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
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetRealObject", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::MemberInfoSerializationHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

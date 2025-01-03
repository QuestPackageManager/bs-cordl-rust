#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberInfoSerializationHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_reflectedType: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    pub m_signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_signature2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_memberType: crate::System::Reflection::MemberTypes,
    pub m_info: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationInfo,
    >,
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::MemberInfoSerializationHolder => "System.Reflection"
    ."MemberInfoSerializationHolder"
);
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl std::ops::Deref for crate::System::Reflection::MemberInfoSerializationHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetRealObject", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializationInfo_Il2CppString_MemberTypes_Il2CppArray1(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        reflectedClass: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        signature2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::System::Reflection::MemberTypes,
        genericArguments: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSerializationInfo",
                (
                    info,
                    name,
                    reflectedClass,
                    signature,
                    signature2,
                    _cordl_type,
                    genericArguments,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializationInfo_MemberTypes0(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        reflectedClass: quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: crate::System::Reflection::MemberTypes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetSerializationInfo",
                (info, name, reflectedClass, signature, _cordl_type),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl AsRef<crate::System::Runtime::Serialization::IObjectReference>
for crate::System::Reflection::MemberInfoSerializationHolder {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::IObjectReference {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl AsMut<crate::System::Runtime::Serialization::IObjectReference>
for crate::System::Reflection::MemberInfoSerializationHolder {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IObjectReference {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::MemberInfoSerializationHolder {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+MemberInfoSerializationHolder")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::MemberInfoSerializationHolder {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}

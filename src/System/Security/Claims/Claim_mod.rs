#[cfg(feature = "System+Security+Claims+Claim")]
#[repr(C)]
#[derive(Debug)]
pub struct Claim {
    __cordl_parent: crate::System::Object,
    pub m_issuer: *mut crate::System::String,
    pub m_originalIssuer: *mut crate::System::String,
    pub m_type: *mut crate::System::String,
    pub m_value: *mut crate::System::String,
    pub m_valueType: *mut crate::System::String,
    pub m_userSerializationData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub m_properties: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub m_propertyLock: *mut crate::System::Object,
    pub m_subject: *mut crate::System::Security::Claims::ClaimsIdentity,
}
#[cfg(feature = "System+Security+Claims+Claim")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Claims::Claim =>
    "System.Security.Claims"."Claim"
);
#[cfg(feature = "System+Security+Claims+Claim")]
impl std::ops::Deref for crate::System::Security::Claims::Claim {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+Claim")]
impl std::ops::DerefMut for crate::System::Security::Claims::Claim {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+Claim")]
impl crate::System::Security::Claims::Claim {
    pub fn Clone(
        &mut self,
        identity: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::Claims::Claim> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Claims::Claim = __cordl_object
            .invoke("Clone", (identity))?;
        Ok(__cordl_ret)
    }
    pub fn New_Claim_ClaimsIdentity2(
        other: *mut crate::System::Security::Claims::Claim,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other, subject))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String_String_String_ClaimsIdentity0(
        _cordl_type: *mut crate::System::String,
        value: *mut crate::System::String,
        valueType: *mut crate::System::String,
        issuer: *mut crate::System::String,
        originalIssuer: *mut crate::System::String,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, value, valueType, issuer, originalIssuer, subject),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String_String_String_ClaimsIdentity_String_String1(
        _cordl_type: *mut crate::System::String,
        value: *mut crate::System::String,
        valueType: *mut crate::System::String,
        issuer: *mut crate::System::String,
        originalIssuer: *mut crate::System::String,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
        propertyKey: *mut crate::System::String,
        propertyValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_type,
                    value,
                    valueType,
                    issuer,
                    originalIssuer,
                    subject,
                    propertyKey,
                    propertyValue,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn OnDeserializedMethod(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializedMethod", (context))?;
        Ok(__cordl_ret)
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
    pub fn _ctor_Claim_ClaimsIdentity2(
        &mut self,
        other: *mut crate::System::Security::Claims::Claim,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other, subject))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_String_String_String_ClaimsIdentity0(
        &mut self,
        _cordl_type: *mut crate::System::String,
        value: *mut crate::System::String,
        valueType: *mut crate::System::String,
        issuer: *mut crate::System::String,
        originalIssuer: *mut crate::System::String,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, value, valueType, issuer, originalIssuer, subject),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_String_String_String_ClaimsIdentity_String_String1(
        &mut self,
        _cordl_type: *mut crate::System::String,
        value: *mut crate::System::String,
        valueType: *mut crate::System::String,
        issuer: *mut crate::System::String,
        originalIssuer: *mut crate::System::String,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
        propertyKey: *mut crate::System::String,
        propertyValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_type,
                    value,
                    valueType,
                    issuer,
                    originalIssuer,
                    subject,
                    propertyKey,
                    propertyValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Subject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Claims::ClaimsIdentity,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Claims::ClaimsIdentity = __cordl_object
            .invoke("get_Subject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Subject(
        &mut self,
        value: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Subject", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Claims+Claim")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::Claims::Claim {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

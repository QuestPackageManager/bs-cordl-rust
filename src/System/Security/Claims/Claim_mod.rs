#[cfg(feature = "System+Security+Claims+Claim")]
#[repr(C)]
#[derive(Debug)]
pub struct Claim {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_issuer: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_originalIssuer: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_type: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_value: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_valueType: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_userSerializationData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub m_properties: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_propertyLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_subject: *mut crate::System::Security::Claims::ClaimsIdentity,
}
#[cfg(feature = "System+Security+Claims+Claim")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Claims::Claim =>
    "System.Security.Claims"."Claim"
);
#[cfg(feature = "System+Security+Claims+Claim")]
impl std::ops::Deref for crate::System::Security::Claims::Claim {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        identity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Claims::Claim>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::Claim,
        > = __cordl_object.invoke("Clone", (identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Claim_ClaimsIdentity2(
        other: quest_hook::libil2cpp::Gc<crate::System::Security::Claims::Claim>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other, subject))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_ClaimsIdentity0(
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        issuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        originalIssuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, value, valueType, issuer, originalIssuer, subject),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_ClaimsIdentity_Il2CppString_Il2CppString1(
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        issuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        originalIssuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
        propertyKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propertyValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
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
    pub fn _ctor_Claim_ClaimsIdentity2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::System::Security::Claims::Claim>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_ClaimsIdentity0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        issuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        originalIssuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, value, valueType, issuer, originalIssuer, subject),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString_Il2CppString_Il2CppString_ClaimsIdentity_Il2CppString_Il2CppString1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        valueType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        issuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        originalIssuer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
        propertyKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        propertyValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_Properties", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Subject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Claims::ClaimsIdentity>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        > = __cordl_object.invoke("get_Subject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Subject(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Security::Claims::ClaimsIdentity>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Subject", (value))?;
        Ok(__cordl_ret.into())
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

#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct ClaimsIdentity {
    __cordl_parent: crate::System::Object,
    pub m_userSerializationData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub m_instanceClaims: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Security::Claims::Claim,
    >,
    pub m_externalClaims: *mut crate::System::Collections::ObjectModel::Collection_1<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
    >,
    pub m_nameType: *mut crate::System::String,
    pub m_roleType: *mut crate::System::String,
    pub m_version: *mut crate::System::String,
    pub m_actor: *mut crate::System::Security::Claims::ClaimsIdentity,
    pub m_authenticationType: *mut crate::System::String,
    pub m_bootstrapContext: *mut crate::System::Object,
    pub m_label: *mut crate::System::String,
    pub m_serializedNameType: *mut crate::System::String,
    pub m_serializedRoleType: *mut crate::System::String,
    pub m_serializedClaims: *mut crate::System::String,
}
#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Claims::ClaimsIdentity =>
    "System.Security.Claims"."ClaimsIdentity"
);
#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
impl std::ops::Deref for crate::System::Security::Claims::ClaimsIdentity {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
impl std::ops::DerefMut for crate::System::Security::Claims::ClaimsIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
impl crate::System::Security::Claims::ClaimsIdentity {
    #[cfg(feature = "System+Security+Claims+ClaimsIdentity+_get_Claims_d__51")]
    pub type _get_Claims_d__51 = crate::System::Security::Claims::ClaimsIdentity__get_Claims_d__51;
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AuthenticationType", ())?;
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
    pub fn set_Actor(
        &mut self,
        value: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Actor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AddClaim(
        &mut self,
        claim: *mut crate::System::Security::Claims::Claim,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddClaim", (claim))?;
        Ok(__cordl_ret)
    }
    pub fn OnSerializingMethod(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializingMethod", (context))?;
        Ok(__cordl_ret)
    }
    pub fn get_NameClaimType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_NameClaimType", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_1(
        &mut self,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (claims))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IIdentity_IEnumerable_1_String_String_String2(
        &mut self,
        identity: *mut crate::System::Security::Principal::IIdentity,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
        authenticationType: *mut crate::System::String,
        nameType: *mut crate::System::String,
        roleType: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (identity, claims, authenticationType, nameType, roleType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IIdentity_IEnumerable_1_String_String_String__cordl_bool3(
        &mut self,
        identity: *mut crate::System::Security::Principal::IIdentity,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
        authenticationType: *mut crate::System::String,
        nameType: *mut crate::System::String,
        roleType: *mut crate::System::String,
        checkAuthType: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (identity, claims, authenticationType, nameType, roleType, checkAuthType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ClaimsIdentity4(
        &mut self,
        other: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext5(
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
    pub fn get_Actor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Claims::ClaimsIdentity,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Claims::ClaimsIdentity = __cordl_object
            .invoke("get_Actor", ())?;
        Ok(__cordl_ret)
    }
    pub fn SafeAddClaims(
        &mut self,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeAddClaims", (claims))?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserializingMethod(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializingMethod", (context))?;
        Ok(__cordl_ret)
    }
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Claims::ClaimsIdentity,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Claims::ClaimsIdentity = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeClaims(
        &mut self,
        serializedClaims: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeClaims", (serializedClaims))?;
        Ok(__cordl_ret)
    }
    pub fn IsCircular(
        &mut self,
        subject: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCircular", (subject))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        useContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (info, context, useContext))?;
        Ok(__cordl_ret)
    }
    pub fn get_Claims(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        > = __cordl_object.invoke("get_Claims", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindFirst(
        &mut self,
        _cordl_type: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Security::Claims::Claim> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Claims::Claim = __cordl_object
            .invoke("FindFirst", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeClaims(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("SerializeClaims", ())?;
        Ok(__cordl_ret)
    }
    pub fn SafeAddClaim(
        &mut self,
        claim: *mut crate::System::Security::Claims::Claim,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SafeAddClaim", (claim))?;
        Ok(__cordl_ret)
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IEnumerable_1_1(
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (claims))?;
        Ok(__cordl_object)
    }
    pub fn New_IIdentity_IEnumerable_1_String_String_String2(
        identity: *mut crate::System::Security::Principal::IIdentity,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
        authenticationType: *mut crate::System::String,
        nameType: *mut crate::System::String,
        roleType: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (identity, claims, authenticationType, nameType, roleType),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_IIdentity_IEnumerable_1_String_String_String__cordl_bool3(
        identity: *mut crate::System::Security::Principal::IIdentity,
        claims: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Security::Claims::Claim,
        >,
        authenticationType: *mut crate::System::String,
        nameType: *mut crate::System::String,
        roleType: *mut crate::System::String,
        checkAuthType: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (identity, claims, authenticationType, nameType, roleType, checkAuthType),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ClaimsIdentity4(
        other: *mut crate::System::Security::Claims::ClaimsIdentity,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext5(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Claims::ClaimsIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

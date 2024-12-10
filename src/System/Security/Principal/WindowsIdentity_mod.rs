#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
#[repr(C)]
#[derive(Debug)]
pub struct WindowsIdentity {
    __cordl_parent: crate::System::Security::Claims::ClaimsIdentity,
    pub _token: crate::System::IntPtr,
    pub _type: *mut quest_hook::libil2cpp::Il2CppString,
    pub _account: crate::System::Security::Principal::WindowsAccountType,
    pub _authenticated: bool,
    pub _name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _info: *mut crate::System::Runtime::Serialization::SerializationInfo,
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Principal::WindowsIdentity =>
    "System.Security.Principal"."WindowsIdentity"
);
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl std::ops::Deref for crate::System::Security::Principal::WindowsIdentity {
    type Target = crate::System::Security::Claims::ClaimsIdentity;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl std::ops::DerefMut for crate::System::Security::Principal::WindowsIdentity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl crate::System::Security::Principal::WindowsIdentity {
    pub fn CloneAsBase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Claims::ClaimsIdentity>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        > = __cordl_object.invoke("CloneAsBase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTokenInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetTokenInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Impersonate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::WindowsImpersonationContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Principal::WindowsImpersonationContext,
        > = __cordl_object.invoke("Impersonate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ClaimsIdentity_IntPtr2(
        claimsIdentity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
        userToken: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (claimsIdentity, userToken))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IntPtr_Il2CppString_WindowsAccountType__cordl_bool0(
        userToken: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        acctType: crate::System::Security::Principal::WindowsAccountType,
        isAuthenticated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userToken, _cordl_type, acctType, isAuthenticated))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext1(
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
    pub fn SetToken(
        &mut self,
        token: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
                (sender),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
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
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ClaimsIdentity_IntPtr2(
        &mut self,
        claimsIdentity: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        >,
        userToken: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (claimsIdentity, userToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr_Il2CppString_WindowsAccountType__cordl_bool0(
        &mut self,
        userToken: crate::System::IntPtr,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        acctType: crate::System::Security::Principal::WindowsAccountType,
        isAuthenticated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (userToken, _cordl_type, acctType, isAuthenticated))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
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
    pub fn get_AuthenticationType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AuthenticationType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Principal::WindowsIdentity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsRef<crate::System::IDisposable>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsMut<crate::System::IDisposable>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsRef<crate::System::Security::Principal::IIdentity>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_ref(&self) -> &crate::System::Security::Principal::IIdentity {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Principal+WindowsIdentity")]
impl AsMut<crate::System::Security::Principal::IIdentity>
for crate::System::Security::Principal::WindowsIdentity {
    fn as_mut(&mut self) -> &mut crate::System::Security::Principal::IIdentity {
        unsafe { std::mem::transmute(self) }
    }
}

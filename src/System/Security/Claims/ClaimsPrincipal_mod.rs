#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
#[repr(C)]
#[derive(Debug)]
pub struct ClaimsPrincipal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_serializedClaimsIdentities: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_identities: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Security::Claims::ClaimsIdentity>,
        >,
    >,
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Claims::ClaimsPrincipal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Claims";
    const CLASS_NAME: &'static str = "ClaimsPrincipal";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl std::ops::Deref for crate::System::Security::Claims::ClaimsPrincipal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl std::ops::DerefMut for crate::System::Security::Claims::ClaimsPrincipal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl crate::System::Security::Claims::ClaimsPrincipal {
    pub fn Deserialize(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "Deserialize", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeIdentities(
        &mut self,
        identities: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DeserializeIdentities")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "DeserializeIdentities",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (identities))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    pub fn OnDeserializedMethod(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Runtime::Serialization::StreamingContext),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnDeserializedMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "OnDeserializedMethod",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSerializingMethod(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Runtime::Serialization::StreamingContext),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnSerializingMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "OnSerializingMethod",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectPrimaryIdentity(
        identities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Claims::ClaimsIdentity,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::Claims::ClaimsIdentity>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Security::Claims::ClaimsIdentity,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Claims::ClaimsIdentity,
                >,
                1usize,
            >("SelectPrimaryIdentity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "SelectPrimaryIdentity",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Claims::ClaimsIdentity,
        > = unsafe { method.invoke_unchecked((), (identities))? };
        Ok(__cordl_ret.into())
    }
    pub fn SerializeIdentities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("SerializeIdentities")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), "SerializeIdentities",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ClaimsPrincipalSelector() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Claims::ClaimsPrincipal,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Claims::ClaimsPrincipal as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Func_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Security::Claims::ClaimsPrincipal,
                        >,
                    >,
                >,
                0usize,
            >("get_ClaimsPrincipalSelector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Security::Claims::ClaimsPrincipal as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_ClaimsPrincipalSelector", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Claims::ClaimsPrincipal,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Claims::ClaimsPrincipal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl AsRef<crate::System::Security::Principal::IPrincipal>
for crate::System::Security::Claims::ClaimsPrincipal {
    fn as_ref(&self) -> &crate::System::Security::Principal::IPrincipal {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Security+Claims+ClaimsPrincipal")]
impl AsMut<crate::System::Security::Principal::IPrincipal>
for crate::System::Security::Claims::ClaimsPrincipal {
    fn as_mut(&mut self) -> &mut crate::System::Security::Principal::IPrincipal {
        unsafe { std::mem::transmute(self) }
    }
}

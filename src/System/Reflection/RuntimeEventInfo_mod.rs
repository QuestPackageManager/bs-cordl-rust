#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeEventInfo {
    __cordl_parent: crate::System::Reflection::EventInfo,
    pub klass: crate::System::IntPtr,
    pub handle: crate::System::IntPtr,
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Reflection::RuntimeEventInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Reflection";
    const CLASS_NAME: &'static str = "RuntimeEventInfo";
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
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl std::ops::Deref for crate::System::Reflection::RuntimeEventInfo {
    type Target = crate::System::Reflection::EventInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl std::ops::DerefMut for crate::System::Reflection::RuntimeEventInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl crate::System::Reflection::RuntimeEventInfo {
    pub fn GetAddMethod(
        &mut self,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                1usize,
            >("GetAddMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetAddMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { method.invoke_unchecked(self, (nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBindingFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Reflection::BindingFlags,
                0usize,
            >("GetBindingFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetBindingFlags", 0usize
                )
            });
        let __cordl_ret: crate::System::Reflection::BindingFlags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                2usize,
            >("GetCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetCustomAttributes",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, (attributeType, inherit))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCustomAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                1usize,
            >("GetCustomAttributes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetCustomAttributes",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, (inherit))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDeclaringTypeInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
                0usize,
            >("GetDeclaringTypeInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetDeclaringTypeInternal",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetEventInfo(
        ev: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::MonoEventInfo> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>),
                crate::System::Reflection::MonoEventInfo,
                1usize,
            >("GetEventInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetEventInfo", 1usize
                )
            });
        let __cordl_ret: crate::System::Reflection::MonoEventInfo = unsafe {
            method.invoke_unchecked((), (ev))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationInfo,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GetObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetObjectData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (info, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRaiseMethod(
        &mut self,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                1usize,
            >("GetRaiseMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetRaiseMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { method.invoke_unchecked(self, (nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemoveMethod(
        &mut self,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                1usize,
            >("GetRemoveMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetRemoveMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = unsafe { method.invoke_unchecked(self, (nonPublic))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRuntimeModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeModule>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeModule>,
                0usize,
            >("GetRuntimeModule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "GetRuntimeModule", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::RuntimeModule,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDefined(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>, bool),
                bool,
                2usize,
            >("IsDefined")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "IsDefined", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (attributeType, inherit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BindingFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Reflection::BindingFlags,
                0usize,
            >("get_BindingFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_BindingFlags", 0usize
                )
            });
        let __cordl_ret: crate::System::Reflection::BindingFlags = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DeclaringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("get_DeclaringType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_DeclaringType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MetadataToken(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_MetadataToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_MetadataToken", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Module(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Module>,
                0usize,
            >("get_Module")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_Module", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Reflection::Module> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_Name")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_Name", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReflectedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                0usize,
            >("get_ReflectedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_ReflectedType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReflectedTypeInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::RuntimeType>,
                0usize,
            >("get_ReflectedTypeInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_ReflectedTypeInternal",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::RuntimeType> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_event_info(
        ev: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>,
        info: quest_hook::libil2cpp::ByRefMut<crate::System::Reflection::MonoEventInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Reflection::RuntimeEventInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::System::Reflection::MonoEventInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("get_event_info")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_event_info", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ev, info))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_metadata_token(
        monoEvent: quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Reflection::RuntimeEventInfo as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Reflection::RuntimeEventInfo>),
                i32,
                1usize,
            >("get_metadata_token")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Reflection::RuntimeEventInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_metadata_token", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (monoEvent))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::RuntimeEventInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::RuntimeEventInfo {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+RuntimeEventInfo")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Reflection::RuntimeEventInfo {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}

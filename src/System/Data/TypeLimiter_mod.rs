#[cfg(feature = "System+Data+TypeLimiter")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeLimiter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_instanceScope: quest_hook::libil2cpp::Gc<
        crate::System::Data::TypeLimiter_Scope,
    >,
}
#[cfg(feature = "System+Data+TypeLimiter")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::TypeLimiter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "TypeLimiter";
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
#[cfg(feature = "System+Data+TypeLimiter")]
impl std::ops::Deref for crate::System::Data::TypeLimiter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl std::ops::DerefMut for crate::System::Data::TypeLimiter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl crate::System::Data::TypeLimiter {
    #[cfg(feature = "System+Data+TypeLimiter+Scope")]
    pub type Scope = crate::System::Data::TypeLimiter_Scope;
    pub fn Capture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
                0usize,
            >("Capture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "Capture", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter> = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTypeIsAllowed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        capturedLimiter: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("EnsureTypeIsAllowed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "EnsureTypeIsAllowed", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_type, capturedLimiter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnterRestrictedScope_DataSet0(
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                1usize,
            >("EnterRestrictedScope")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "EnterRestrictedScope", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked((), (dataSet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnterRestrictedScope_DataTable1(
        dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
                1usize,
            >("EnterRestrictedScope")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "EnterRestrictedScope", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = unsafe {
            method.invoke_unchecked((), (dataTable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviouslyDeclaredDataTypes_DataSet1(
        dataSet: quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataSet>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    >,
                >,
                1usize,
            >("GetPreviouslyDeclaredDataTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "GetPreviouslyDeclaredDataTypes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = unsafe { method.invoke_unchecked((), (dataSet))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreviouslyDeclaredDataTypes_DataTable0(
        dataTable: quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::DataTable>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    >,
                >,
                1usize,
            >("GetPreviouslyDeclaredDataTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "GetPreviouslyDeclaredDataTypes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = unsafe { method.invoke_unchecked((), (dataTable))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        scope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (scope))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        scope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (scope))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsTypeLimitingDisabled() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsTypeLimitingDisabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsTypeLimitingDisabled", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+TypeLimiter")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::TypeLimiter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeLimiter_Scope {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_allowedTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub m_previousScope: quest_hook::libil2cpp::Gc<
        crate::System::Data::TypeLimiter_Scope,
    >,
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::TypeLimiter_Scope {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "TypeLimiter/Scope";
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
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl std::ops::Deref for crate::System::Data::TypeLimiter_Scope {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl std::ops::DerefMut for crate::System::Data::TypeLimiter_Scope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl crate::System::Data::TypeLimiter_Scope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter_Scope as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter_Scope as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAllowedType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter_Scope as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsAllowedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter_Scope as
                    quest_hook::libil2cpp::Type > ::class(), "IsAllowedType", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeUnconditionallyAllowed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter_Scope as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("IsTypeUnconditionallyAllowed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter_Scope as
                    quest_hook::libil2cpp::Type > ::class(),
                    "IsTypeUnconditionallyAllowed", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        previousScope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
        allowedTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (previousScope, allowedTypes))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        previousScope: quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
        allowedTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Data::TypeLimiter_Scope as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Data::TypeLimiter_Scope>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Data::TypeLimiter_Scope as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (previousScope, allowedTypes))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::TypeLimiter_Scope {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl AsRef<crate::System::IDisposable> for crate::System::Data::TypeLimiter_Scope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+TypeLimiter+Scope")]
impl AsMut<crate::System::IDisposable> for crate::System::Data::TypeLimiter_Scope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}

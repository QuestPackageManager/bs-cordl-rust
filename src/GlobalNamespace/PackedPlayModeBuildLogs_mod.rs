#[cfg(feature = "PackedPlayModeBuildLogs")]
#[repr(C)]
#[derive(Debug)]
pub struct PackedPlayModeBuildLogs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_RuntimeBuildLogs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
        >,
    >,
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PackedPlayModeBuildLogs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PackedPlayModeBuildLogs";
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
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl std::ops::Deref for crate::GlobalNamespace::PackedPlayModeBuildLogs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackedPlayModeBuildLogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl crate::GlobalNamespace::PackedPlayModeBuildLogs {
    #[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
    pub type RuntimeBuildLog = crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RuntimeBuildLogs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
            >,
        > = __cordl_object.invoke("get_RuntimeBuildLogs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RuntimeBuildLogs(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RuntimeBuildLogs", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackedPlayModeBuildLogs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackedPlayModeBuildLogs_RuntimeBuildLog {
    pub Type: crate::UnityEngine::LogType,
    pub Message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RuntimeBuildLog";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PackedPlayModeBuildLogs+RuntimeBuildLog")]
impl crate::GlobalNamespace::PackedPlayModeBuildLogs_RuntimeBuildLog {
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::UnityEngine::LogType,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type, message),
        )?;
        Ok(__cordl_ret.into())
    }
}

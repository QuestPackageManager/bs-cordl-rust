#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
#[repr(C)]
#[derive(Debug)]
pub struct SemVer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Major: i32,
    pub m_Minor: i32,
    pub m_Patch: i32,
    pub m_Build: i32,
    pub m_Type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SemVer =>
    "UnityEngine.ProBuilder"."SemVer"
);
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SemVer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SemVer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl crate::UnityEngine::ProBuilder::SemVer {
    pub const DefaultStringFormat: &'static str = "M.m.p-t.b";
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_SemVer1(
        &mut self,
        version: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_SemVer1(
        &mut self,
        version: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBuildNumber(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBuildNumber", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString1(
        formatted: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (formatted, date))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_i32_Il2CppString_Il2CppString_Il2CppString2(
        major: i32,
        minor: i32,
        patch: i32,
        build: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (major, minor, patch, build, _cordl_type, date, metadata),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToString_1(
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
    pub fn ToString_Il2CppString0(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetVersionInfo(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::ProBuilder::SemVer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetVersionInfo", (input, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn WrapNoValue(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WrapNoValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString1(
        &mut self,
        formatted: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (formatted, date))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_Il2CppString_Il2CppString_Il2CppString2(
        &mut self,
        major: i32,
        minor: i32,
        patch: i32,
        build: i32,
        _cordl_type: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        date: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        metadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (major, minor, patch, build, _cordl_type, date, metadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MajorMinorPatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::SemVer,
        > = __cordl_object.invoke("get_MajorMinorPatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_build(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_date", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_major(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_major", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_metadata(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_metadata", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minor(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_minor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_patch(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_patch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SemVer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::SemVer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsRef<crate::System::IComparable> for crate::UnityEngine::ProBuilder::SemVer {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsMut<crate::System::IComparable> for crate::UnityEngine::ProBuilder::SemVer {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsRef<crate::System::IComparable_1<*mut crate::UnityEngine::ProBuilder::SemVer>>
for crate::UnityEngine::ProBuilder::SemVer {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<*mut crate::UnityEngine::ProBuilder::SemVer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsMut<crate::System::IComparable_1<*mut crate::UnityEngine::ProBuilder::SemVer>>
for crate::UnityEngine::ProBuilder::SemVer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<*mut crate::UnityEngine::ProBuilder::SemVer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsRef<crate::System::IEquatable_1<*mut crate::UnityEngine::ProBuilder::SemVer>>
for crate::UnityEngine::ProBuilder::SemVer {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<*mut crate::UnityEngine::ProBuilder::SemVer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SemVer")]
impl AsMut<crate::System::IEquatable_1<*mut crate::UnityEngine::ProBuilder::SemVer>>
for crate::UnityEngine::ProBuilder::SemVer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<*mut crate::UnityEngine::ProBuilder::SemVer> {
        unsafe { std::mem::transmute(self) }
    }
}

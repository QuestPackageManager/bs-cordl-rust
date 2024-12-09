#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVersionPropertyBag {
    __cordl_parent: crate::Unity::Properties::ContainerPropertyBag_1<
        *mut crate::System::Version,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::SystemVersionPropertyBag =>
    "Unity.Properties.Internal"."SystemVersionPropertyBag"
);
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
impl std::ops::Deref for crate::Unity::Properties::Internal::SystemVersionPropertyBag {
    type Target = crate::Unity::Properties::ContainerPropertyBag_1<
        *mut crate::System::Version,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::SystemVersionPropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
impl crate::Unity::Properties::Internal::SystemVersionPropertyBag {
    #[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
    pub type BuildProperty = crate::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty;
    #[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
    pub type MajorProperty = crate::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty;
    #[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
    pub type MinorProperty = crate::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty;
    #[cfg(
        feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty"
    )]
    pub type RevisionProperty = crate::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::SystemVersionPropertyBag {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVersionPropertyBag_BuildProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        *mut crate::System::Version,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty =>
    "Unity.Properties.Internal"."SystemVersionPropertyBag/BuildProperty"
);
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty {
    type Target = crate::Unity::Properties::Property_2<*mut crate::System::Version, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
impl crate::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+BuildProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_BuildProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVersionPropertyBag_MajorProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        *mut crate::System::Version,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty =>
    "Unity.Properties.Internal"."SystemVersionPropertyBag/MajorProperty"
);
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty {
    type Target = crate::Unity::Properties::Property_2<*mut crate::System::Version, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
impl crate::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MajorProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MajorProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVersionPropertyBag_MinorProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        *mut crate::System::Version,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty =>
    "Unity.Properties.Internal"."SystemVersionPropertyBag/MinorProperty"
);
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty {
    type Target = crate::Unity::Properties::Property_2<*mut crate::System::Version, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
impl crate::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+MinorProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_MinorProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemVersionPropertyBag_RevisionProperty {
    __cordl_parent: crate::Unity::Properties::Property_2<
        *mut crate::System::Version,
        i32,
    >,
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty =>
    "Unity.Properties.Internal"."SystemVersionPropertyBag/RevisionProperty"
);
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty {
    type Target = crate::Unity::Properties::Property_2<*mut crate::System::Version, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
impl crate::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+Internal+SystemVersionPropertyBag+RevisionProperty")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::SystemVersionPropertyBag_RevisionProperty {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

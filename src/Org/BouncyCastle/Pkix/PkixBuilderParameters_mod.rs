#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixBuilderParameters {
    __cordl_parent: crate::Org::BouncyCastle::Pkix::PkixParameters,
    pub maxPathLength: i32,
    pub excludedCerts: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixBuilderParameters
    => "Org.BouncyCastle.Pkix"."PkixBuilderParameters"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixBuilderParameters {
    type Target = crate::Org::BouncyCastle::Pkix::PkixParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixBuilderParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
impl crate::Org::BouncyCastle::Pkix::PkixBuilderParameters {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExcludedCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("GetExcludedCerts", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        trustAnchors: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        targetConstraints: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trustAnchors, targetConstraints))?;
        Ok(__cordl_object)
    }
    pub fn SetExcludedCerts(
        &mut self,
        excludedCerts: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExcludedCerts", (excludedCerts))?;
        Ok(__cordl_ret)
    }
    pub fn SetParams(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Pkix::PkixParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParams", (parameters))?;
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
    pub fn _ctor(
        &mut self,
        trustAnchors: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        targetConstraints: *mut crate::Org::BouncyCastle::X509::Store::IX509Selector,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trustAnchors, targetConstraints))?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxPathLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxPathLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_MaxPathLength(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxPathLength", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixBuilderParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
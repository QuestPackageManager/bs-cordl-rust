#[cfg(feature = "Org+BouncyCastle+Pkix+PkixBuilderParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixBuilderParameters {
    __cordl_parent: crate::Org::BouncyCastle::Pkix::PkixParameters,
    pub maxPathLength: i32,
    pub excludedCerts: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Collections::ISet,
    >,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExcludedCerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = __cordl_object.invoke("GetExcludedCerts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance(
        pkixParams: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixBuilderParameters>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (pkixParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        trustAnchors: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        targetConstraints: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Selector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (trustAnchors, targetConstraints))?;
        Ok(__cordl_object.into())
    }
    pub fn SetExcludedCerts(
        &mut self,
        excludedCerts: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExcludedCerts", (excludedCerts))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetParams(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParams", (parameters))?;
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
    pub fn _ctor(
        &mut self,
        trustAnchors: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        targetConstraints: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::X509::Store::IX509Selector,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (trustAnchors, targetConstraints))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxPathLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxPathLength", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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

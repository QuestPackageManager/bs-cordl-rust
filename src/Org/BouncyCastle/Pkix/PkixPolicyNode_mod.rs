#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixPolicyNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mChildren: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    pub mDepth: i32,
    pub mExpectedPolicies: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Collections::ISet,
    >,
    pub mParent: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    >,
    pub mPolicyQualifiers: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Collections::ISet,
    >,
    pub mValidPolicy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub mCritical: bool,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixPolicyNode =>
    "Org.BouncyCastle.Pkix"."PkixPolicyNode"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixPolicyNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixPolicyNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
impl crate::Org::BouncyCastle::Pkix::PkixPolicyNode {
    pub fn AddChild(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChild", (child))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = __cordl_object.invoke("Copy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        children: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        depth: i32,
        expectedPolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        parent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyQualifiers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        validPolicy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    children,
                    depth,
                    expectedPolicies,
                    parent,
                    policyQualifiers,
                    validPolicy,
                    critical,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveChild(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveChild", (child))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
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
    pub fn ToString_Il2CppString1(
        &mut self,
        indent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (indent))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        children: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        depth: i32,
        expectedPolicies: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        parent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        >,
        policyQualifiers: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
        validPolicy: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    children,
                    depth,
                    expectedPolicies,
                    parent,
                    policyQualifiers,
                    validPolicy,
                    critical,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_Children", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExpectedPolicies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = __cordl_object.invoke("get_ExpectedPolicies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasChildren(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasChildren", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCritical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCritical", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        > = __cordl_object.invoke("get_Parent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PolicyQualifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::Collections::ISet>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        > = __cordl_object.invoke("get_PolicyQualifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValidPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ValidPolicy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExpectedPolicies(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Collections::ISet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExpectedPolicies", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsCritical(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsCritical", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Parent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Pkix::PkixPolicyNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Parent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixPolicyNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

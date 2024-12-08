#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixPolicyNode {
    __cordl_parent: crate::System::Object,
    pub mChildren: *mut crate::System::Collections::IList,
    pub mDepth: i32,
    pub mExpectedPolicies: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub mParent: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    pub mPolicyQualifiers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    pub mValidPolicy: *mut crate::System::String,
    pub mCritical: bool,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixPolicyNode =>
    "Org.BouncyCastle.Pkix"."PkixPolicyNode"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixPolicyNode")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixPolicyNode {
    type Target = crate::System::Object;
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
        child: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChild", (child))?;
        Ok(__cordl_ret)
    }
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
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode = __cordl_object
            .invoke("Copy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        children: *mut crate::System::Collections::IList,
        depth: i32,
        expectedPolicies: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        parent: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        policyQualifiers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        validPolicy: *mut crate::System::String,
        critical: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn RemoveChild(
        &mut self,
        child: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveChild", (child))?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String1(
        &mut self,
        indent: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (indent))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        children: *mut crate::System::Collections::IList,
        depth: i32,
        expectedPolicies: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        parent: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
        policyQualifiers: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
        validPolicy: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn get_Children(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("get_Children", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Depth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Depth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExpectedPolicies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("get_ExpectedPolicies", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasChildren(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCritical(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCritical", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Parent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode = __cordl_object
            .invoke("get_Parent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PolicyQualifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("get_PolicyQualifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValidPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ValidPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ExpectedPolicies(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ExpectedPolicies", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_Parent(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Pkix::PkixPolicyNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Parent", (value))?;
        Ok(__cordl_ret)
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

#[cfg(feature = "ModestTree+Util+UnityUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::Util::UnityUtil => "ModestTree.Util"
    ."UnityUtil"
);
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl std::ops::Deref for crate::ModestTree::Util::UnityUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl std::ops::DerefMut for crate::ModestTree::Util::UnityUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl crate::ModestTree::Util::UnityUtil {
    #[cfg(feature = "ModestTree+Util+UnityUtil+_GetParents_d__16")]
    pub type _GetParents_d__16 = crate::ModestTree::Util::UnityUtil__GetParents_d__16;
    #[cfg(feature = "ModestTree+Util+UnityUtil+_GetDirectChildren_d__21")]
    pub type _GetDirectChildren_d__21 = crate::ModestTree::Util::UnityUtil__GetDirectChildren_d__21;
    #[cfg(feature = "ModestTree+Util+UnityUtil+__c")]
    pub type __c = crate::ModestTree::Util::UnityUtil___c;
    #[cfg(feature = "ModestTree+Util+UnityUtil+_get_AllScenes_d__1")]
    pub type _get_AllScenes_d__1 = crate::ModestTree::Util::UnityUtil__get_AllScenes_d__1;
    #[cfg(feature = "ModestTree+Util+UnityUtil+_GetParentsAndSelf_d__17")]
    pub type _GetParentsAndSelf_d__17 = crate::ModestTree::Util::UnityUtil__GetParentsAndSelf_d__17;
    #[cfg(feature = "ModestTree+Util+UnityUtil+_GetDirectChildrenAndSelf_d__20")]
    pub type _GetDirectChildrenAndSelf_d__20 = crate::ModestTree::Util::UnityUtil__GetDirectChildrenAndSelf_d__20;
}
#[cfg(feature = "ModestTree+Util+UnityUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Util::UnityUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

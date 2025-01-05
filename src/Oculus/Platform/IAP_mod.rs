#[cfg(feature = "Oculus+Platform+IAP")]
#[repr(C)]
#[derive(Debug)]
pub struct IAP {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Oculus+Platform+IAP")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::IAP => "Oculus.Platform"."IAP"
);
#[cfg(feature = "Oculus+Platform+IAP")]
impl std::ops::Deref for crate::Oculus::Platform::IAP {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+IAP")]
impl std::ops::DerefMut for crate::Oculus::Platform::IAP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+IAP")]
impl crate::Oculus::Platform::IAP {
    pub fn ConsumePurchase(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConsumePurchase", (sku))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextProductListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextProductListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextPurchaseListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextPurchaseListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProductsBySKU(
        skus: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProductsBySKU", (skus))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetViewerPurchases() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetViewerPurchases", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetViewerPurchasesDurableCache() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetViewerPurchasesDurableCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchCheckoutFlow(
        sku: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Purchase>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Purchase>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchCheckoutFlow", (sku))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+IAP")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::IAP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

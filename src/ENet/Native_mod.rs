#[cfg(feature = "ENet+Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Native {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ENet+Native")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::Native => "ENet"."Native"
);
#[cfg(feature = "ENet+Native")]
impl std::ops::Deref for crate::ENet::Native {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Native")]
impl std::ops::DerefMut for crate::ENet::Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Native")]
impl crate::ENet::Native {
    pub const cryptoNativeLibrary: &'static str = "crypto";
    pub const nativeLibrary: &'static str = "enet";
    pub const sslNativeLibrary: &'static str = "ssl";
    pub fn OPENSSL_init_crypto(
        opts: u64,
        settings: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OPENSSL_init_crypto", (opts, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn OPENSSL_init_ssl(
        opts: u64,
        settings: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OPENSSL_init_ssl", (opts, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_address_get_hostname(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        hostName: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        nameLength: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_address_get_hostname", (address, hostName, nameLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_address_get_ip(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        ip: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        ipLength: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_address_get_ip", (address, ip, ipLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_address_set_hostname(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_address_set_hostname", (address, hostName))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_address_set_ip(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        ip: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_address_set_ip", (address, ip))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_crc64(
        buffers: crate::System::IntPtr,
        bufferCount: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_crc64", (buffers, bufferCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_deinitialize() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_deinitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_bandwidth_limit(
        host: crate::System::IntPtr,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_bandwidth_limit",
                (host, incomingBandwidth, outgoingBandwidth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_broadcast(
        host: crate::System::IntPtr,
        channelID: u8,
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_broadcast", (host, channelID, packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_broadcast_exclude(
        host: crate::System::IntPtr,
        channelID: u8,
        packet: crate::System::IntPtr,
        excludedPeer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_broadcast_exclude",
                (host, channelID, packet, excludedPeer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_broadcast_selective(
        host: crate::System::IntPtr,
        channelID: u8,
        packet: crate::System::IntPtr,
        peers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
        >,
        peersLength: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_broadcast_selective",
                (host, channelID, packet, peers, peersLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_channel_limit(
        host: crate::System::IntPtr,
        channelLimit: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_channel_limit", (host, channelLimit))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_check_events(
        host: crate::System::IntPtr,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetEvent>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_check_events", (host, event))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_connect(
        host: crate::System::IntPtr,
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        channelCount: crate::System::IntPtr,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_connect", (host, address, channelCount, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_create_ByRefMut0(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        peerLimit: crate::System::IntPtr,
        channelLimit: crate::System::IntPtr,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_create",
                (
                    address,
                    peerLimit,
                    channelLimit,
                    incomingBandwidth,
                    outgoingBandwidth,
                    bufferSize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_create_IntPtr1(
        address: crate::System::IntPtr,
        peerLimit: crate::System::IntPtr,
        channelLimit: crate::System::IntPtr,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_create",
                (
                    address,
                    peerLimit,
                    channelLimit,
                    incomingBandwidth,
                    outgoingBandwidth,
                    bufferSize,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_create_ssl_ByRefMut0(
        address: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetAddress>,
        peerLimit: crate::System::IntPtr,
        channelLimit: crate::System::IntPtr,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
        bufferSize: i32,
        sslConfiguration: quest_hook::libil2cpp::ByRefMut<
            crate::ENet::ENetSslConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_create_ssl",
                (
                    address,
                    peerLimit,
                    channelLimit,
                    incomingBandwidth,
                    outgoingBandwidth,
                    bufferSize,
                    sslConfiguration,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_create_ssl_IntPtr1(
        address: crate::System::IntPtr,
        peerLimit: crate::System::IntPtr,
        channelLimit: crate::System::IntPtr,
        incomingBandwidth: u32,
        outgoingBandwidth: u32,
        bufferSize: i32,
        sslConfiguration: quest_hook::libil2cpp::ByRefMut<
            crate::ENet::ENetSslConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_host_create_ssl",
                (
                    address,
                    peerLimit,
                    channelLimit,
                    incomingBandwidth,
                    outgoingBandwidth,
                    bufferSize,
                    sslConfiguration,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_destroy(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_destroy", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_flush(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_flush", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_get_bytes_received(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_get_bytes_received", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_get_bytes_sent(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_get_bytes_sent", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_get_packets_received(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_get_packets_received", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_get_packets_sent(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_get_packets_sent", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_get_peers_count(
        host: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_get_peers_count", (host))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_prevent_connections(
        host: crate::System::IntPtr,
        state: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_prevent_connections", (host, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_service(
        host: crate::System::IntPtr,
        event: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetEvent>,
        timeout: u32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_service", (host, event, timeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_set_checksum_callback(
        host: crate::System::IntPtr,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_set_checksum_callback", (host, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_set_intercept_callback(
        host: crate::System::IntPtr,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_set_intercept_callback", (host, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_host_set_max_duplicate_peers(
        host: crate::System::IntPtr,
        number: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_host_set_max_duplicate_peers", (host, number))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_initialize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_initialize_with_callbacks(
        version: u32,
        inits: quest_hook::libil2cpp::ByRefMut<crate::ENet::ENetCallbacks>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_initialize_with_callbacks", (version, inits))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_linked_version() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_linked_version", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_check_references(
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_check_references", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_create_Gc0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataLength: crate::System::IntPtr,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_create", (data, dataLength, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_create_IntPtr1(
        data: crate::System::IntPtr,
        dataLength: crate::System::IntPtr,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_create", (data, dataLength, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_create_offset_Gc0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dataLength: crate::System::IntPtr,
        dataOffset: crate::System::IntPtr,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_create_offset", (data, dataLength, dataOffset, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_create_offset_IntPtr1(
        data: crate::System::IntPtr,
        dataLength: crate::System::IntPtr,
        dataOffset: crate::System::IntPtr,
        flags: crate::ENet::PacketFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_create_offset", (data, dataLength, dataOffset, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_dispose(
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_dispose", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_get_data(
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_get_data", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_get_length(
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_get_length", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_get_user_data(
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_get_user_data", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_set_free_callback(
        packet: crate::System::IntPtr,
        callback: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_set_free_callback", (packet, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_packet_set_user_data(
        packet: crate::System::IntPtr,
        userData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_packet_set_user_data", (packet, userData))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_disconnect(
        peer: crate::System::IntPtr,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_disconnect", (peer, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_disconnect_later(
        peer: crate::System::IntPtr,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_disconnect_later", (peer, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_disconnect_now(
        peer: crate::System::IntPtr,
        data: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_disconnect_now", (peer, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_bytes_received(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_bytes_received", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_bytes_sent(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_bytes_sent", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_data(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_data", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_id(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_id", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_ip(
        peer: crate::System::IntPtr,
        ip: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ipLength: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_ip", (peer, ip, ipLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_last_rtt(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_last_rtt", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_lastreceivetime(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_lastreceivetime", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_lastsendtime(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_lastsendtime", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_mtu(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_mtu", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_packets_lost(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_packets_lost", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_packets_sent(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_packets_sent", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_packets_throttle(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_packets_throttle", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_port(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_port", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_rtt(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_rtt", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_get_state(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::ENet::PeerState> {
        let __cordl_ret: crate::ENet::PeerState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_get_state", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_ping(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_ping", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_ping_interval(
        peer: crate::System::IntPtr,
        pingInterval: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_ping_interval", (peer, pingInterval))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_receive(
        peer: crate::System::IntPtr,
        channelID: quest_hook::libil2cpp::ByRefMut<u8>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_receive", (peer, channelID))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_reset(
        peer: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_reset", (peer))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_send(
        peer: crate::System::IntPtr,
        channelID: u8,
        packet: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_send", (peer, channelID, packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_set_data(
        peer: crate::System::IntPtr,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_peer_set_data", (peer, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_throttle_configure(
        peer: crate::System::IntPtr,
        interval: u32,
        acceleration: u32,
        deceleration: u32,
        threshold: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_peer_throttle_configure",
                (peer, interval, acceleration, deceleration, threshold),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_peer_timeout(
        peer: crate::System::IntPtr,
        timeoutLimit: u32,
        timeoutMinimum: u32,
        timeoutMaximum: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "enet_peer_timeout",
                (peer, timeoutLimit, timeoutMinimum, timeoutMaximum),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn enet_time_get() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("enet_time_get", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ENet+Native")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}

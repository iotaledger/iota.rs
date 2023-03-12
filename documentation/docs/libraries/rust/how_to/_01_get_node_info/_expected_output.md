```json
NodeInfoWrapper {
    node_info: InfoResponse {
        name: "HORNET",
        version: "2.0.0-beta.5",
        status: StatusResponse {
            is_healthy: true,
            latest_milestone: LatestMilestoneResponse {
                index: 746869,
                timestamp: 1661868434,
                milestone_id: "0xbab289ebb1fdcc8516772074c3c465aaad648f234a4c1ff138f16c65a88b1298",
            },
            confirmed_milestone: ConfirmedMilestoneResponse {
                index: 746869,
                timestamp: 1661868434,
                milestone_id: "0xbab289ebb1fdcc8516772074c3c465aaad648f234a4c1ff138f16c65a88b1298",
            },
            pruning_index: 0,
        },
        supported_protocol_versions: [
            2,
        ],
        protocol: ProtocolResponse {
            version: 2,
            network_name: "testnet-1",
            bech32_hrp: "rms",
            min_pow_score: 1500.0,
            rent_structure: RentStructureResponse {
                v_byte_cost: 100,
                v_byte_factor_key: 10,
                v_byte_factor_data: 1,
            },
            token_supply: "1450896407249092",
        },
        pending_protocol_parameters: [],
        base_token: BaseTokenResponse {
            name: "Shimmer",
            ticker_symbol: "SMR",
            unit: "SMR",
            subunit: Some(
                "glow",
            ),
            decimals: 6,
            use_metric_prefix: false,
        },
        metrics: MetricsResponse {
            blocks_per_second: 8.4,
            referenced_blocks_per_second: 9.2,
            referenced_rate: 109.52380952380952,
        },
        features: [],
    },
    url: "https://api.testnet.shimmer.network",
}
```
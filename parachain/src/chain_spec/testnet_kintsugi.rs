use super::*;

fn testnet_properties(bitcoin_network: &str) -> Map<String, Value> {
    let mut properties = Map::new();
    let mut token_symbol: Vec<String> = vec![];
    let mut token_decimals: Vec<u32> = vec![];
    [KINT, KBTC, KSM, INTR, IBTC, DOT].iter().for_each(|token| {
        token_symbol.push(token.symbol().to_string());
        token_decimals.push(token.decimals() as u32);
    });
    properties.insert("tokenSymbol".into(), token_symbol.into());
    properties.insert("tokenDecimals".into(), token_decimals.into());
    properties.insert("ss58Format".into(), testnet_kintsugi_runtime::SS58Prefix::get().into());
    properties.insert("bitcoinNetwork".into(), bitcoin_network.into());
    properties
}

fn default_pair_testnet(currency_id: CurrencyId) -> VaultCurrencyPair<CurrencyId> {
    VaultCurrencyPair {
        collateral: currency_id,
        wrapped: testnet_kintsugi_runtime::GetWrappedCurrencyId::get(),
    }
}

pub fn local_config(id: ParaId) -> KintsugiTestnetChainSpec {
    KintsugiTestnetChainSpec::from_genesis(
        "interBTC",
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![get_authority_keys_from_seed("Alice")],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    "Bob".as_bytes().to_vec(),
                )],
                id,
                DEFAULT_BITCOIN_CONFIRMATIONS,
                false,
            )
        },
        vec![],
        None,
        None,
        None,
        Some(testnet_properties(BITCOIN_REGTEST)),
        Extensions {
            relay_chain: "local".into(),
            para_id: id.into(),
        },
    )
}

pub fn development_config(id: ParaId) -> KintsugiTestnetChainSpec {
    KintsugiTestnetChainSpec::from_genesis(
        "interBTC",
        "dev_testnet",
        ChainType::Development,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                vec![get_authority_keys_from_seed("Alice")],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        "Alice".as_bytes().to_vec(),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        "Bob".as_bytes().to_vec(),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        "Charlie".as_bytes().to_vec(),
                    ),
                ],
                id,
                DEFAULT_BITCOIN_CONFIRMATIONS,
                false,
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(testnet_properties(BITCOIN_REGTEST)),
        Extensions {
            relay_chain: "dev".into(),
            para_id: id.into(),
        },
    )
}

pub fn staging_testnet_config(id: ParaId) -> KintsugiTestnetChainSpec {
    KintsugiTestnetChainSpec::from_genesis(
        "interBTC",
        "staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                // 5Ec37KSdjSbGKoQN4evLXrZskjc7jxXYrowPHEtH2MzRC7mv (//sudo/1)
                get_account_id_from_string("5Ec37KSdjSbGKoQN4evLXrZskjc7jxXYrowPHEtH2MzRC7mv"),
                vec![
                    // 5EqCiRZGFZ88JCK9FNmak2SkRHSohWpEFpx28vwo5c1m98Xe (//authority/1)
                    get_authority_keys_from_public_key(hex![
                        "7a6868acf544dc5c3f2f9f6f9a5952017bbefb51da41819307fc21cf3efb554d"
                    ]),
                    // 5DbwRgYTAtjJ8Mys8ta8RXxWPcSmiyx4dPRsvU1k4TYyk4jq (//authority/2)
                    get_authority_keys_from_public_key(hex![
                        "440e84dd3604be606f3110c21f93a0e981fb93b28288270dcdce8a43c68ff36e"
                    ]),
                    // 5GVtSRJmnFxVcFz7jejbCrY2SREhZJZUHuJkm2KS75bTqRF2 (//authority/3)
                    get_authority_keys_from_public_key(hex![
                        "c425b0d9fed64d3bd5be0a6d06053d2bfb72f4983146788f5684aec9f5eb0c7f"
                    ]),
                ],
                vec![
                    // 5Ec37KSdjSbGKoQN4evLXrZskjc7jxXYrowPHEtH2MzRC7mv (//sudo/1)
                    get_account_id_from_string("5Ec37KSdjSbGKoQN4evLXrZskjc7jxXYrowPHEtH2MzRC7mv"),
                    // 5ECj4iBBi3h8kYzhqLFmzVLafC64UpsXvK7H4ZZyXoVQJdJq (//oracle/1)
                    get_account_id_from_string("5ECj4iBBi3h8kYzhqLFmzVLafC64UpsXvK7H4ZZyXoVQJdJq"),
                    // 5FgWDuxgS8VasP6KtvESHUuuDn6L8BTCqbYyFW9mDwAaLtbY (//account/1)
                    get_account_id_from_string("5FgWDuxgS8VasP6KtvESHUuuDn6L8BTCqbYyFW9mDwAaLtbY"),
                    // 5H3n25VshwPeMzKhn4gnVEjCEndFsjt85ydW2Vvo8ysy7CnZ (//account/2)
                    get_account_id_from_string("5H3n25VshwPeMzKhn4gnVEjCEndFsjt85ydW2Vvo8ysy7CnZ"),
                    // 5GKciEHZWSGxtAihqGjXC6XpXSGNoudDxACuDLbYF1ipygZj (//account/3)
                    get_account_id_from_string("5GKciEHZWSGxtAihqGjXC6XpXSGNoudDxACuDLbYF1ipygZj"),
                    // 5GjJ26ffHApgUFLgxKWpWL5T5ppxWjSRJe42PjPNATLvjcJK (//account/4)
                    get_account_id_from_string("5GjJ26ffHApgUFLgxKWpWL5T5ppxWjSRJe42PjPNATLvjcJK"),
                    // 5DqzGaydetDXGya818gyuHA7GAjEWRsQN6UWNKpvfgq2KyM7 (//account/5)
                    get_account_id_from_string("5DqzGaydetDXGya818gyuHA7GAjEWRsQN6UWNKpvfgq2KyM7"),
                ],
                vec![(
                    // 5ECj4iBBi3h8kYzhqLFmzVLafC64UpsXvK7H4ZZyXoVQJdJq (//oracle/1)
                    get_account_id_from_string("5ECj4iBBi3h8kYzhqLFmzVLafC64UpsXvK7H4ZZyXoVQJdJq"),
                    "Interlay".as_bytes().to_vec(),
                )],
                id,
                DEFAULT_BITCOIN_CONFIRMATIONS,
                false,
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(testnet_properties(BITCOIN_TESTNET)),
        Extensions {
            relay_chain: "staging".into(),
            para_id: id.into(),
        },
    )
}

pub fn rococo_testnet_config(id: ParaId) -> KintsugiTestnetChainSpec {
    KintsugiTestnetChainSpec::from_genesis(
        "interBTC",
        "rococo_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                // 5E4hDxbuLqzpAhcEsqaJKULgkTcEfzAqsbEQLV471cDC2Hhx (//sudo/1)
                get_account_id_from_string("5E4hDxbuLqzpAhcEsqaJKULgkTcEfzAqsbEQLV471cDC2Hhx"),
                vec![
                    // 5GELfhX7eEeJfXuSe3NdfVyj13yKYegBtg8BLPQxeKDbAwzd (//authority/1)
                    get_authority_keys_from_public_key(hex![
                        "b84a0f13ef5eb4d7c1caf735081bd2c91667b84f4b18cd7fa176a73ffd36c133"
                    ]),
                    // 5Et1qfhf6zNmgZF7JWYApngE4HCxw2SxZTWKLEMZ73cFBnh6 (//authority/2)
                    get_authority_keys_from_public_key(hex![
                        "7c8d8946973c243888a4eba8f34288cc9f26a3b0f7114b932d6fde362ad67034"
                    ]),
                    // 5Cw1w8J8W8grtyWLUT8bs7GtjCm483pGT1ym8Q6K3HVaRcWb (//authority/3)
                    get_authority_keys_from_public_key(hex![
                        "265f1f526a9360030fcb0780ca597e398930cd9571f161b67d33d2bdd9957024"
                    ]),
                ],
                vec![
                    // 5E4hDxbuLqzpAhcEsqaJKULgkTcEfzAqsbEQLV471cDC2Hhx (//sudo/1)
                    get_account_id_from_string("5E4hDxbuLqzpAhcEsqaJKULgkTcEfzAqsbEQLV471cDC2Hhx"),
                    // 5FKuXEdswjda6EfXtWcTbdVH8vQbmNDWhK2qrPGx6GeHvvZh (//oracle/1)
                    get_account_id_from_string("5FKuXEdswjda6EfXtWcTbdVH8vQbmNDWhK2qrPGx6GeHvvZh"),
                    // 5CRrztZ1XYGBZ2asHJFD81W1vSpWiDqq8ndGJmLpRQboeMjM (//account/1)
                    get_account_id_from_string("5CRrztZ1XYGBZ2asHJFD81W1vSpWiDqq8ndGJmLpRQboeMjM"),
                    // 5GjX9J4w1QkfbzoeRL9Uv2JjLg7DkcJfFt4CnKYcPtgkXtmb (//account/2)
                    get_account_id_from_string("5GjX9J4w1QkfbzoeRL9Uv2JjLg7DkcJfFt4CnKYcPtgkXtmb"),
                    // 5GNTqNZL5ADeHRML85C5Y7tdDCZiiXbN3JJNEZvKJXVbyHUT (//account/3)
                    get_account_id_from_string("5GNTqNZL5ADeHRML85C5Y7tdDCZiiXbN3JJNEZvKJXVbyHUT"),
                    // 5HjPDnGAx3opfZtu3wKiZ7BYXXAjEgjwKiufXtZfesTCMgmP (//account/4)
                    get_account_id_from_string("5HjPDnGAx3opfZtu3wKiZ7BYXXAjEgjwKiufXtZfesTCMgmP"),
                    // 5GuXvbk5MaXvm9enTocGmzF8L7T6djzgt4T29SGAFDvLHmAL (//account/5)
                    get_account_id_from_string("5GuXvbk5MaXvm9enTocGmzF8L7T6djzgt4T29SGAFDvLHmAL"),
                ],
                vec![(
                    // 5FKuXEdswjda6EfXtWcTbdVH8vQbmNDWhK2qrPGx6GeHvvZh (//oracle/1)
                    get_account_id_from_string("5FKuXEdswjda6EfXtWcTbdVH8vQbmNDWhK2qrPGx6GeHvvZh"),
                    "Interlay".as_bytes().to_vec(),
                )],
                id,
                DEFAULT_BITCOIN_CONFIRMATIONS,
                false,
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(testnet_properties(BITCOIN_TESTNET)),
        Extensions {
            relay_chain: "rococo".into(),
            para_id: id.into(),
        },
    )
}

pub fn rococo_local_testnet_config(id: ParaId) -> KintsugiTestnetChainSpec {
    development_config(id)
}

pub fn westend_testnet_config(id: ParaId) -> KintsugiTestnetChainSpec {
    KintsugiTestnetChainSpec::from_genesis(
        "interBTC",
        "westend_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                // 5DjsgavDiY8xMcR4riDvs9JXYUpCMnHBe45xsA1rPeBD5woG (//sudo/1)
                get_account_id_from_string("5DjsgavDiY8xMcR4riDvs9JXYUpCMnHBe45xsA1rPeBD5woG"),
                vec![
                    // 5FxMV7qEw3h5yJkrbxUtW18FzU7jhBzeCHfbLB1CDJ1ikyVY (//authority/1)
                    get_authority_keys_from_public_key(hex![
                        "ac18e27687e17fe0a7fc49e3c4bf22673b5beb4f38fa950e62ec4105e9842714"
                    ]),
                    // 5Cr6MMKUAbKSzhmLmi9RRNeMfkh7eMXS3Ya11mBTSTRQGBTu (//authority/2)
                    get_authority_keys_from_public_key(hex![
                        "229dc43a3b9647a4c8b1aa44b1655ea8655f00c44740ec6bb8e45a628fc99a7c"
                    ]),
                    // 5FHuLURhM4aDXy7Rd6e4Lbbg9H7fbQcUutMbRviaPjCi5SZt (//authority/3)
                    get_authority_keys_from_public_key(hex![
                        "8ec588a0de7ba6e877c676e2f276254f8033141df8ee9fad66f89090c6c3b376"
                    ]),
                ],
                vec![
                    // 5DjsgavDiY8xMcR4riDvs9JXYUpCMnHBe45xsA1rPeBD5woG (//sudo/1)
                    get_account_id_from_string("5DjsgavDiY8xMcR4riDvs9JXYUpCMnHBe45xsA1rPeBD5woG"),
                    // 5DMALjH2zJXa4YgG33J2YFBHKeWeP6M7pHugEi5Bk8Qda6bs (//oracle/1)
                    get_account_id_from_string("5DMALjH2zJXa4YgG33J2YFBHKeWeP6M7pHugEi5Bk8Qda6bs"),
                    // 5ENdYBBpnXWMcufn84g6zNevaKdsuFzyCPJu9zG8q6jqwZPu (//account/1)
                    get_account_id_from_string("5ENdYBBpnXWMcufn84g6zNevaKdsuFzyCPJu9zG8q6jqwZPu"),
                    // 5ECo5XVKPRwMu1Zue9deUChx4VmJiaUz5JY4fVFa7zWz555D (//account/2)
                    get_account_id_from_string("5ECo5XVKPRwMu1Zue9deUChx4VmJiaUz5JY4fVFa7zWz555D"),
                    // 5CDjUcujZfXmJv4cmP5cUS7N96yiJNfN9ScTE1QHDak3vEnD (//account/3)
                    get_account_id_from_string("5CDjUcujZfXmJv4cmP5cUS7N96yiJNfN9ScTE1QHDak3vEnD"),
                    // 5FRnXtLTLNbGuEF63YkqLwEeeDh1xtuaCy6Qp3VEUZErJa4M (//account/4)
                    get_account_id_from_string("5FRnXtLTLNbGuEF63YkqLwEeeDh1xtuaCy6Qp3VEUZErJa4M"),
                    // 5CAep2mugERSXpCQTWT5i9vLJXtF1L7CqwpKhVBrmwKsix4A (//account/5)
                    get_account_id_from_string("5CAep2mugERSXpCQTWT5i9vLJXtF1L7CqwpKhVBrmwKsix4A"),
                ],
                vec![(
                    // 5DMALjH2zJXa4YgG33J2YFBHKeWeP6M7pHugEi5Bk8Qda6bs (//oracle/1)
                    get_account_id_from_string("5DMALjH2zJXa4YgG33J2YFBHKeWeP6M7pHugEi5Bk8Qda6bs"),
                    "Interlay".as_bytes().to_vec(),
                )],
                id,
                DEFAULT_BITCOIN_CONFIRMATIONS,
                false,
            )
        },
        Vec::new(),
        None,
        None,
        None,
        Some(testnet_properties(BITCOIN_TESTNET)),
        Extensions {
            relay_chain: "westend".into(),
            para_id: id.into(),
        },
    )
}

fn testnet_genesis(
    root_key: AccountId,
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    authorized_oracles: Vec<(AccountId, Vec<u8>)>,
    id: ParaId,
    bitcoin_confirmations: u32,
    start_shutdown: bool,
) -> testnet_kintsugi_runtime::GenesisConfig {
    testnet_kintsugi_runtime::GenesisConfig {
        system: testnet_kintsugi_runtime::SystemConfig {
            code: testnet_kintsugi_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        parachain_system: Default::default(),
        parachain_info: testnet_kintsugi_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: testnet_kintsugi_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: Zero::zero(),
            ..Default::default()
        },
        session: testnet_kintsugi_runtime::SessionConfig {
            keys: invulnerables
                .iter()
                .cloned()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                                    // account id
                        acc.clone(),                                    // validator id
                        testnet_kintsugi_runtime::SessionKeys { aura }, // session keys
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do.
        // Session will take care of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        security: testnet_kintsugi_runtime::SecurityConfig {
            initial_status: if start_shutdown {
                testnet_kintsugi_runtime::StatusCode::Shutdown
            } else {
                testnet_kintsugi_runtime::StatusCode::Error
            },
        },
        sudo: testnet_kintsugi_runtime::SudoConfig {
            // Assign network admin rights.
            key: Some(root_key.clone()),
        },
        asset_registry: Default::default(),
        tokens: testnet_kintsugi_runtime::TokensConfig {
            balances: endowed_accounts
                .iter()
                .flat_map(|k| {
                    vec![
                        (k.clone(), Token(DOT), 1 << 60),
                        (k.clone(), Token(INTR), 1 << 60),
                        (k.clone(), Token(KSM), 1 << 60),
                        (k.clone(), Token(KINT), 1 << 60),
                    ]
                })
                .collect(),
        },
        vesting: Default::default(),
        oracle: testnet_kintsugi_runtime::OracleConfig {
            authorized_oracles,
            max_delay: DEFAULT_MAX_DELAY_MS,
        },
        btc_relay: testnet_kintsugi_runtime::BTCRelayConfig {
            bitcoin_confirmations,
            parachain_confirmations: bitcoin_confirmations
                .saturating_mul(testnet_kintsugi_runtime::BITCOIN_BLOCK_SPACING),
            disable_difficulty_check: true,
            disable_inclusion_check: false,
        },
        issue: testnet_kintsugi_runtime::IssueConfig {
            issue_period: testnet_kintsugi_runtime::DAYS * 2,
            issue_btc_dust_value: DEFAULT_DUST_VALUE,
        },
        redeem: testnet_kintsugi_runtime::RedeemConfig {
            redeem_transaction_size: expected_transaction_size(),
            redeem_period: testnet_kintsugi_runtime::DAYS * 2,
            redeem_btc_dust_value: DEFAULT_DUST_VALUE,
        },
        replace: testnet_kintsugi_runtime::ReplaceConfig {
            replace_period: testnet_kintsugi_runtime::DAYS * 2,
            replace_btc_dust_value: DEFAULT_DUST_VALUE,
        },
        vault_registry: testnet_kintsugi_runtime::VaultRegistryConfig {
            minimum_collateral_vault: vec![(Token(KINT), 55 * KINT.one()), (Token(KSM), 3 * KSM.one())],
            punishment_delay: kintsugi_runtime::DAYS,
            system_collateral_ceiling: vec![
                (default_pair_testnet(Token(KINT)), 26_200 * KINT.one()),
                (default_pair_testnet(Token(KSM)), 60_000 * KSM.one()),
            ],
            secure_collateral_threshold: vec![
                (
                    default_pair_testnet(Token(KINT)),
                    /* 900% */
                    FixedU128::checked_from_rational(900, 100).unwrap(),
                ),
                (
                    default_pair_testnet(Token(KSM)),
                    /* 260% */
                    FixedU128::checked_from_rational(260, 100).unwrap(),
                ),
            ],
            premium_redeem_threshold: vec![
                (
                    default_pair_testnet(Token(KINT)),
                    /* 650% */
                    FixedU128::checked_from_rational(650, 100).unwrap(),
                ),
                (
                    default_pair_testnet(Token(KSM)),
                    /* 200% */
                    FixedU128::checked_from_rational(200, 100).unwrap(),
                ),
            ],
            liquidation_collateral_threshold: vec![
                (
                    default_pair_testnet(Token(KINT)),
                    /* 500% */
                    FixedU128::checked_from_rational(500, 100).unwrap(),
                ),
                (
                    default_pair_testnet(Token(KSM)),
                    /* 150% */
                    FixedU128::checked_from_rational(150, 100).unwrap(),
                ),
            ],
        },
        fee: testnet_kintsugi_runtime::FeeConfig {
            issue_fee: FixedU128::checked_from_rational(15, 10000).unwrap(), // 0.15%
            issue_griefing_collateral: FixedU128::checked_from_rational(5, 100000).unwrap(), // 0.005%
            redeem_fee: FixedU128::checked_from_rational(5, 1000).unwrap(),  // 0.5%
            premium_redeem_fee: FixedU128::checked_from_rational(5, 100).unwrap(), // 5%
            punishment_fee: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
            replace_griefing_collateral: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
        },
        nomination: testnet_kintsugi_runtime::NominationConfig {
            is_nomination_enabled: false,
        },
        technical_committee: Default::default(),
        technical_membership: Default::default(),
        treasury: Default::default(),
        democracy: Default::default(),
        supply: testnet_kintsugi_runtime::SupplyConfig {
            initial_supply: testnet_kintsugi_runtime::token_distribution::INITIAL_ALLOCATION,
            // start of year 5
            start_height: testnet_kintsugi_runtime::YEARS * 4,
            inflation: FixedU128::checked_from_rational(2, 100).unwrap(), // 2%
        },
        polkadot_xcm: testnet_kintsugi_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(2),
        },
    }
}

pub mod commands;
pub use commands::*;

#[cfg(test)]
mod tests {
    use crate::commands;

    #[test]
    fn execute() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let program_id = "credlancer_rating_survey_v2.aleo";
        let function = "submit_assessor";
        let inputs = vec![
            "10000field".to_string(),
            "1field".to_string(),
            "1u64".to_string(),
        ];
        let record =  Some("{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}");
        let fee = Some(3000u64);
        let query = Some("https://api.explorer.aleo.org/v1");

        let transaction = commands::execute(
            private_key,
            program_id,
            function,
            inputs,
            record,
            fee,
            query,
        );

        println!("execute transaction: {}", transaction.unwrap());
    }

    #[test]
    fn transfer() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let recipient = "aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz";
        let amount = 100000u64;
        let function = "private_to_public";
        let input_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let query = None;

        let transaction = commands::transfer(
            private_key,
            recipient,
            amount,
            function,
            Some(input_record),
            Some(fee_record),
            None,
            query,
        );
        println!("transfer transaction: {}", transaction.unwrap());
    }

    #[test]
    fn join() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let first_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 39349297u64.private,  _nonce: 1711234411548659851426643372350679804994150752915608367928992063815077333263group.public}";
        let second_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let fee_record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 2417370u64.private,  _nonce: 6125567204338138114699824935057483504900942792237093903423312221865875650403group.public}";

        let transaction = commands::join(
            private_key,
            first_record,
            second_record,
            Some(fee_record),
            None,
            None,
        );
        println!("join transaction: {}", transaction.unwrap());
    }

    #[test]
    fn split() {
        let private_key = "APrivateKey1zkp5jS79CdFE5LbUzTvXcAd4fZLCTUVVqqgD91AApZVXJcA";
        let record = "{  owner: aleo1y3yp6kaq4tl04u06fy4k43kvcl4azlddy0fsamdj6smsa6rnxg9sk09ltz.private,  microcredits: 4593678u64.private,  _nonce: 2519444983733166664452332514213249618009422832813505241649029942240273681354group.public}";
        let amount = 1000000u64;
        let transaction = commands::split(private_key, record, amount, None);
        println!("join transaction: {}", transaction.unwrap());
    }

    #[test]
    fn execution_cost() {

        // let ss = commands::execution_cost(program_id, function, inputs, query)
    }

    #[test]
    fn transaction_for_authorize() {
        let execute_authorization = r#"
        {
            "requests": [
              {
                "signer": "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                "network": "3u16",
                "program": "ans_registrar_v3.aleo",
                "function": "register_fld",
                "input_ids": [
                  {
                    "type": "private",
                    "id": "5823803650542459922433101942332685166655249362364364643243144233987401103760field"
                  },
                  {
                    "type": "private",
                    "id": "4448380391514448580625116991301766058022755381083333101675569613596096391879field"
                  },
                  {
                    "type": "external_record",
                    "id": "8184545075958142391627282974802298987853783943948558008988702449177789503622field"
                  }
                ],
                "inputs": [
                  "[126939695768936u128,0u128,0u128,0u128]",
                  "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                  "{owner:aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns.private,microcredits:15000000u64.private,_nonce:4746724296554693491974932956926959977437892695259873352519368910338565912297group.public}"
                ],
                "signature": "sign1kmjxjtp36s2m94awlmpqfxkvz5vpumw84pnmtkg8dk3uskgk65pxlq5a9p55vhk4nfaeapjp60myqkuteejgyrvqy0hqr6an0d5jqqq32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsjqlcmx0",
                "sk_tag": "8392991161530139852499528373585292010003647568520892791643421634970601392470field",
                "tvk": "2566519709986503158313657644739652072579454072479088806231329597947760239613field",
                "tcm": "3493378742611950205623944304676334606161896139822026602850632541552825449154field"
              },
              {
                "signer": "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                "network": "3u16",
                "program": "credits.aleo",
                "function": "transfer_private_to_public",
                "input_ids": [
                  {
                    "type": "record",
                    "commitment": "6676205857049259427534171678310610848747918340179868148970671120310959505385field",
                    "gamma": "5802319866000626804407417141548188086142750039932807560684807966265468208191group",
                    "serial_number": "8007571855843463685323070780269431881675165044681984537136043391923638819876field",
                    "tag": "4312550742714886751270213103050599312865622641926968692652674841554915257584field"
                  },
                  {
                    "type": "public",
                    "id": "2931063980519743022306511780197899157445560171762428888785893480329762547651field"
                  },
                  {
                    "type": "public",
                    "id": "2150139040923825812096779360217642218749767801037945551875599645313726783102field"
                  }
                ],
                "inputs": [
                  "{owner:aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns.private,microcredits:15000000u64.private,_nonce:4746724296554693491974932956926959977437892695259873352519368910338565912297group.public}",
                  "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk",
                  "2000000u64"
                ],
                "signature": "sign1zk26525czwqfsjd4ef5khss9jjf6jc4shykl2kxqkkgtktvmhcq7pt7vwu8j5449s52vw3wxt06mq2sclkk9vdxj9n6uuzahxl32gqs32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsja7d3px",
                "sk_tag": "8392991161530139852499528373585292010003647568520892791643421634970601392470field",
                "tvk": "1898794900696248486178395204313179827721650642067991326566034519382750447280field",
                "tcm": "2401258464241367711225581056384633144464519699549713911409537669025170018220field"
              },
              {
                "signer": "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                "network": "3u16",
                "program": "aleo_name_service_registry_v3.aleo",
                "function": "register",
                "input_ids": [
                  {
                    "type": "private",
                    "id": "5854455283839571643710379859528069723530066665381617574961786071480958382121field"
                  },
                  {
                    "type": "private",
                    "id": "1582766122219912547772225747884237157636763630571098116577289847788719568636field"
                  },
                  {
                    "type": "private",
                    "id": "1892527520673472765480005116127138002290567044242979343628423468628063765106field"
                  },
                  {
                    "type": "private",
                    "id": "7489566998833308635355045105540913907530420261460873860369546889752728846608field"
                  }
                ],
                "inputs": [
                  "[126939695768936u128,0u128,0u128,0u128]",
                  "3601410589032411677092457044111621862970800028849492457114786804129430260029field",
                  "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                  "0u128"
                ],
                "signature": "sign1c8zqnu08vdm3tefrqkxhn0527k382t4jmt60kjv4e8mjgl655gpahvqzpzl9520t05k9zenu24tp6enq54tusxqyeej6cl8d64rcvqs32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsjlfdt99",
                "sk_tag": "8392991161530139852499528373585292010003647568520892791643421634970601392470field",
                "tvk": "410378195056953745549190433305648821534540494999420295735424655588108925389field",
                "tcm": "5746708841559749114174161674482312575674507134875991251966792284219080351331field"
              }
            ],
            "transitions": [
              {
                "id": "au15lptmnpz2zg9gp9qxe0008s8mxfn4nmy7fcnmf0h56vkycszp5yswaw23z",
                "program": "ans_registrar_v3.aleo",
                "function": "register_fld",
                "inputs": [
                  {
                    "type": "private",
                    "id": "5823803650542459922433101942332685166655249362364364643243144233987401103760field",
                    "value": "ciphertext1qvqr26t292jqezv059vhhm82846twyvm2mhtzw50m6g4hslxu0a95qqkx68x0j2pjysk7dpv3kgmek2ktutux4e90ta0k3yqj3eutzqqpszwusf5w9pnlq4zjxep4wgrfs9stm4kywxd8956cv92ul5mz8esykwtw7p"
                  },
                  {
                    "type": "private",
                    "id": "4448380391514448580625116991301766058022755381083333101675569613596096391879field",
                    "value": "ciphertext1qgqgu7stdhxj37vy3cy45d83sad2mw548pdneu4lll6t9n2cpyxcuzdyle0vnh5n507xn9kftxf6rxvs66gast9y7z0zyj03t34lafwzpsz25sw8"
                  },
                  {
                    "type": "external_record",
                    "id": "8184545075958142391627282974802298987853783943948558008988702449177789503622field"
                  }
                ],
                "outputs": [
                  {
                    "type": "future",
                    "id": "2579070691434717366265735890821858891205190208798216339446973296635254337027field",
                    "value": "{program_id:ans_registrar_v3.aleo,function_name:register_fld,arguments:[{program_id:credits.aleo,function_name:transfer_private_to_public,arguments:[aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,2000000u64]},{program_id:aleo_name_service_registry_v3.aleo,function_name:register,arguments:[947635870328996654604545301519999035786519991884691456438858806153778394765field,[126939695768936u128,0u128,0u128,0u128],3601410589032411677092457044111621862970800028849492457114786804129430260029field,0u128,aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk]}]}"
                  }
                ],
                "tpk": "2420109804493595179691608713003823636946004373200835231276784406280250783076group",
                "tcm": "3493378742611950205623944304676334606161896139822026602850632541552825449154field"
              },
              {
                "id": "au1l7z5e5a7d7pqjumppn9jtnz4j6aak0jd3vsq549k33g6zpagr5rs7jllvj",
                "program": "credits.aleo",
                "function": "transfer_private_to_public",
                "inputs": [
                  {
                    "type": "record",
                    "id": "8007571855843463685323070780269431881675165044681984537136043391923638819876field",
                    "tag": "4312550742714886751270213103050599312865622641926968692652674841554915257584field"
                  },
                  {
                    "type": "public",
                    "id": "2931063980519743022306511780197899157445560171762428888785893480329762547651field",
                    "value": "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk"
                  },
                  {
                    "type": "public",
                    "id": "2150139040923825812096779360217642218749767801037945551875599645313726783102field",
                    "value": "2000000u64"
                  }
                ],
                "outputs": [
                  {
                    "type": "record",
                    "id": "7252248519507249541810812667545864363445486613442736736716594241536186418892field",
                    "checksum": "4028251789685564816246086401243549809995546657442603184902596641410110605913field",
                    "value": "record1qyqspgxwjsru4meg8xf5aeah9kxdmc93k4knljglxl4wwynn7987lzgzqyxx66trwfhkxun9v35hguerqqpqzqqfkn5wggcf06kasysr4v4p5flnv55n0v4z9n9czuwqcja0gva6pm9ql980qjdr7hs9txs3350m588r35d3k07e057cl75vgwq0nh7qwlrzxu3"
                  },
                  {
                    "type": "future",
                    "id": "6641225490762795507536226713879442926648930224218685910735472839703210059129field",
                    "value": "{program_id:credits.aleo,function_name:transfer_private_to_public,arguments:[aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,2000000u64]}"
                  }
                ],
                "tpk": "3253727266401889011319492572263263144080135247100585930913010483259738246044group",
                "tcm": "2401258464241367711225581056384633144464519699549713911409537669025170018220field"
              },
              {
                "id": "au1jq79cua7q7yy6nk50jqn4ty853jwzv02zvxgcgyr22s568ze8urs55f0sc",
                "program": "aleo_name_service_registry_v3.aleo",
                "function": "register",
                "inputs": [
                  {
                    "type": "private",
                    "id": "5854455283839571643710379859528069723530066665381617574961786071480958382121field",
                    "value": "ciphertext1qvqdsunjy2mf9nrjp7gjqtpxf9cfacyptkvghn2xpw8z3qxt7vqzurvv6d6l0ggksul7qzplpjy33h8tg4gqw6xwk5hgj0hclemjgnahp4al3a37u9pdv9nggrkuk8n4skl4ukxuqv65vu097gtarhc4dqgqytceyas"
                  },
                  {
                    "type": "private",
                    "id": "1582766122219912547772225747884237157636763630571098116577289847788719568636field",
                    "value": "ciphertext1qgqdv4z8c4pwk62d8l03zy7dh2q2jf8wm0mgq5xs4r4yj4j2nff0wpm6xutes98gw86e6hg2gtvrg6ecqcn5zuc7nwat4fqg3qv490xhqvl2ke94"
                  },
                  {
                    "type": "private",
                    "id": "1892527520673472765480005116127138002290567044242979343628423468628063765106field",
                    "value": "ciphertext1qgqznr2p3smzna6rvsk2vfhw9mp8jmw5nxydfzz6vpe8gzkueqg66qucf680e9jt4weyf0de0gpxyq2sdwwjzw72r24rt24zfklpcd4dps4fhers"
                  },
                  {
                    "type": "private",
                    "id": "7489566998833308635355045105540913907530420261460873860369546889752728846608field",
                    "value": "ciphertext1qyqdmzxsmed20u80fkayz7nwuq4u8d9w4jj8d8hn7pu3s0f4tqkkxzcjhg92a"
                  }
                ],
                "outputs": [
                  {
                    "type": "record",
                    "id": "5215342878018450636910355950302288120839533792880009150929278547414666568589field",
                    "checksum": "7603170442200075755736192465128863736903228022044192936986795739712243248390field",
                    "value": "record1qyqsp3fupe745af6naeeza47pj77804x6zwqnjaupvra9fpufmmpapssqgzxgct5v9psqqszqp3yf2h9xxkqd6x5gw9h2l3um39c62790wtp352vm8kp58c8hmmspysazpc70vhdgej3qkxp6c4fpdepsdxg7u2d7dc24v2hveec7scgqajkg6t5d9hkuscqqgpqqg5vg95h20yj7fjtjml303rnu7l848g0unuj38lc8kusu3esrxcwtqflpmvehy3vmva28hdcftlzethcle05llm0znwlrunauny3rgphtp8ahcd5jlc0ve4gl9pvmd7huazgzefks47e33adp6nzsjldyqq84tskw"
                  },
                  {
                    "type": "future",
                    "id": "2444758763997400050621986985690492375978168731345019497521640866494700329187field",
                    "value": "{program_id:aleo_name_service_registry_v3.aleo,function_name:register,arguments:[947635870328996654604545301519999035786519991884691456438858806153778394765field,[126939695768936u128,0u128,0u128,0u128],3601410589032411677092457044111621862970800028849492457114786804129430260029field,0u128,aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk]}"
                  }
                ],
                "tpk": "7588866226616748124688312699078652882635625974565403402656895513592256967440group",
                "tcm": "5746708841559749114174161674482312575674507134875991251966792284219080351331field"
              }
            ]
          }
    "#;
        let fee_authorization = r#"
        {
            "requests": [
                {
                    "signer": "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
                    "network": "3u16",
                    "program": "credits.aleo",
                    "function": "fee_public",
                    "input_ids": [
                        {
                            "type": "public",
                            "id": "3206345399757417690148600956020165621492042310305932323744742406866511888193field"
                        },
                        {
                            "type": "public",
                            "id": "5049391953525132930968361266067413861026992302262459768945373746812127892024field"
                        },
                        {
                            "type": "public",
                            "id": "411569197658577426897396021312813212183850747947150948886128023775480654499field"
                        }
                    ],
                    "inputs": [
                        "364600u64",
                        "10u64",
                        "2626620350831266276570529756016950809315112994082865001766667566553139729069field"
                    ],
                    "signature": "sign1f38hnlxqzg750tw96qgq9ec7kx5lgt4tmry0lzs24lt7fn0fz5qfeeu0hwqgrdwt8c2490af6uf4f0evaxwc5dl7gqldmdl2eghq7qs32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsjkrrchs",
                    "sk_tag": "8392991161530139852499528373585292010003647568520892791643421634970601392470field",
                    "tvk": "536592475964207750072645438996227429325380203467321758443883932030341988759field",
                    "tcm": "3689765519368670746803563628038180377904488730146550328371176026570068762412field"
                }
            ],
            "transitions": [
                {
                    "id": "au1l4ue0kdpkvplpnsqats2e693hmadvx4zy036kzysmxm8g6hd459qtrrh2p",
                    "program": "credits.aleo",
                    "function": "fee_public",
                    "inputs": [
                        {
                            "type": "public",
                            "id": "3206345399757417690148600956020165621492042310305932323744742406866511888193field",
                            "value": "364600u64"
                        },
                        {
                            "type": "public",
                            "id": "5049391953525132930968361266067413861026992302262459768945373746812127892024field",
                            "value": "10u64"
                        },
                        {
                            "type": "public",
                            "id": "411569197658577426897396021312813212183850747947150948886128023775480654499field",
                            "value": "2626620350831266276570529756016950809315112994082865001766667566553139729069field"
                        }
                    ],
                    "outputs": [
                        {
                            "type": "future",
                            "id": "393065511619742962782826627860376532902259030015894103526853200643152345709field",
                            "value": "{\nprogram_id:credits.aleo,\nfunction_name:fee_public,\narguments:[\naleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns,\n364610u64\n]\n}"
                        }
                    ],
                    "tpk": "2140860592189217932300725718471929502056140244429120438185078678926484017521group",
                    "tcm": "3689765519368670746803563628038180377904488730146550328371176026570068762412field"
                }
            ]
        }
    "#;

        let query = Some("https://aleochain.sotertech.io");

        let transaction = commands::transaction_for_authorize(
            "ans_registrar_v3.aleo",
            execute_authorization,
            fee_authorization,
            query,
        );
        println!("transfer transaction: {}", transaction.unwrap());
    }
}

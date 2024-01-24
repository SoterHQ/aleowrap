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
    fn transaction_for_authorize() {
        let execute_authorization = r#"
        {
          "requests": [
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "ans_registrar_v3.aleo",
                  "function": "register_fld",
                  "input_ids": [
                      {
                          "type": "private",
                          "id": "2413109936835612095473873127449289319886127925236694093870280185354707649922field"
                      },
                      {
                          "type": "private",
                          "id": "4054430502716392470361501337301482017782939902452185366321496189067430414156field"
                      },
                      {
                          "type": "external_record",
                          "id": "552768722850909533074524003192605309276916889875240628573948784161143150842field"
                      }
                  ],
                  "inputs": [
                      "[\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n]",
                      "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                      "{\n  owner: aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s.private,\n  microcredits: 3360625u64.private,\n  _nonce: 6755319637731613343169990547003035554909118965409023337724706464194317148546group.public\n}"
                  ],
                  "signature": "sign17pyxvjkhpm9809v4umfxcwyh2rrur6jruhvw4a7smgl6tyfh65pc3j5xyk5p8qsnhcgcvzzf9khhk9xx9skm6cvq9vxfgu4p4rddkqtkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q69vh79t",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "615773232593459454849189940269468037301586945432863192131754508046360314863field",
                  "tcm": "3200235939611944040827854218598157017513384640468128366131444470338593978612field"
              },
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "credits.aleo",
                  "function": "transfer_private_to_public",
                  "input_ids": [
                      {
                          "type": "record",
                          "commitment": "5285743509580651613836085785444392724241985265312438718114876511545363763276field",
                          "gamma": "4939586439829843445888593228166152705538245380517674416688636202354721097782group",
                          "serial_number": "7194047904949744381042008412666492861890446076712460242169866538749591846485field",
                          "tag": "867581519213588558477924408394505423861150311985691906127848578458509312494field"
                      },
                      {
                          "type": "public",
                          "id": "4085992707787846988806173649067715950013245109798517325277546873866667738395field"
                      },
                      {
                          "type": "public",
                          "id": "4201848541613129278080414622873109941689977856740970554686587401648883512222field"
                      }
                  ],
                  "inputs": [
                      "{\n  owner: aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s.private,\n  microcredits: 3360625u64.private,\n  _nonce: 6755319637731613343169990547003035554909118965409023337724706464194317148546group.public\n}",
                      "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk",
                      "2000000u64"
                  ],
                  "signature": "sign1jxypnjrx8s4skg4yuz63zle5pk85np8duc9h9gw8w4efw7jdrqq0kfyfkkcxadf8kcuk66ffzzxr4j0xwdc58u4rt55l2qafs066kqrkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q6g9j8rs",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "2488483095255106678134679844379366627168187953291874874541516448422432108905field",
                  "tcm": "6841255139595749472275446347930895765057286243053856562446133350885139114581field"
              },
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "aleo_name_service_registry_v3.aleo",
                  "function": "register",
                  "input_ids": [
                      {
                          "type": "private",
                          "id": "6909035840792431568942240361481230062968957326807529302867016965729621030786field"
                      },
                      {
                          "type": "private",
                          "id": "5287731742262540871585927936299361359314418854376324674193462218381480740699field"
                      },
                      {
                          "type": "private",
                          "id": "4780648626598733420327901902796317428868812913462539543282206431218858006967field"
                      },
                      {
                          "type": "private",
                          "id": "1051666325121632181583719347829586283933225966064054779411473986216402532999field"
                      }
                  ],
                  "inputs": [
                      "[\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n]",
                      "3601410589032411677092457044111621862970800028849492457114786804129430260029field",
                      "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                      "0u128"
                  ],
                  "signature": "sign1pfaancu02esay79samvn7a6zkywasqxqvhxtqsgyjaqtvzhx3cpnfckhuglcx43q3pk8hcledxljzctmxnd43umavmvjm3kgflzyvqnkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q6hdjq3q",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "2798735880835806033027265130365377522643708207060050646244265139383027748019field",
                  "tcm": "5063886804062189846704902100343637895026967807966020303368241407248708773143field"
              }
          ],
          "transitions": [
              {
                  "id": "au1gk2xraje9m7hp4l073m4nsyc9hp22l9jhcy0whyxdd5ancwxcq8qftq74d",
                  "program": "credits.aleo",
                  "function": "transfer_private_to_public",
                  "inputs": [
                      {
                          "type": "record",
                          "id": "7194047904949744381042008412666492861890446076712460242169866538749591846485field",
                          "tag": "867581519213588558477924408394505423861150311985691906127848578458509312494field"
                      },
                      {
                          "type": "public",
                          "id": "4085992707787846988806173649067715950013245109798517325277546873866667738395field",
                          "value": "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk"
                      },
                      {
                          "type": "public",
                          "id": "4201848541613129278080414622873109941689977856740970554686587401648883512222field",
                          "value": "2000000u64"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "record",
                          "id": "5038802135377020518179838167452708663270964125145877803104694628649134935117field",
                          "checksum": "6220691502790345109534922685261605533680725175185198318951675108525455348773field",
                          "value": "record1qyqsp4t0wv7ap75s5nkxc3z5jan2jhepf9fhsqkqeq6pr6tdmv8c5vqwqyxx66trwfhkxun9v35hguerqqpqzqx56z7l7l3pnj98l9zlyrqf22gknext47jvj4vlsr20ljpsgzguq45vxaswk48gulgpqjuql6n35p0uazdqywu8hzxyaa5vcrc6f8dq7ncxk9j"
                      },
                      {
                          "type": "future",
                          "id": "3237012691650891316823433605543950000833525309701820848507373470024687526235field",
                          "value": "{\n  program_id: credits.aleo,\n  function_name: transfer_private_to_public,\n  arguments: [\n    aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,\n    2000000u64\n  ]\n}"
                      }
                  ],
                  "tpk": "2076819040045510863228558773346133675190800676730155837100903172584756663871group",
                  "tcm": "6841255139595749472275446347930895765057286243053856562446133350885139114581field"
              },
              {
                  "id": "au1jqtgn3j0x2cr2pu5g3sdx8s83anfvu86jrmc48yl2j7udwvn5qpsuafh3q",
                  "program": "aleo_name_service_registry_v3.aleo",
                  "function": "register",
                  "inputs": [
                      {
                          "type": "private",
                          "id": "6909035840792431568942240361481230062968957326807529302867016965729621030786field",
                          "value": "ciphertext1qvqxj92g2h469hx5rx50anqpkpula0j4hdl9jkkctq5pxkcr7gnwyyd5yf7accwqjcxdtc3ggyrw5a99au8tm33eqqspx76q2le9kmp2p9rfq9u2ul55d65px9j2k6743hlp96hhjhxk9qrvfuffxhpw0q8sxmk0qu9"
                      },
                      {
                          "type": "private",
                          "id": "5287731742262540871585927936299361359314418854376324674193462218381480740699field",
                          "value": "ciphertext1qgqzjvdetjjuaz7qpgkthn9p7xck0y82shfkye756y4tezzta8sj7p8tjgtcnpsrt8hkz3ggh2xlpm9t5fhp7qerr2mwlj4af0p00hphzqkmeyrl"
                      },
                      {
                          "type": "private",
                          "id": "4780648626598733420327901902796317428868812913462539543282206431218858006967field",
                          "value": "ciphertext1qgq0dgv5z90alwz5plt88rux3x729249n38efw90ta6cczl2r6v0yrhfc4z468z3ttffqldvmfg8mrju70duegrcm0zk6slff8lezm3tpvx2udmh"
                      },
                      {
                          "type": "private",
                          "id": "1051666325121632181583719347829586283933225966064054779411473986216402532999field",
                          "value": "ciphertext1qyqyu7kac2u9cfc5rtl7ukp2l49lenwksfta0hr9d5jpr2efwj66uqgzmuj97"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "record",
                          "id": "6259488067471386787837075778944647574465061092993325020240541141015439965652field",
                          "checksum": "5164963297651619656029586786429889762272560621538667310903856175441498615580field",
                          "value": "record1qyqspn4mjrcuym8g3e3mfc4s8xk6mplcrnj00n7hmzejwacc8sw3v7gpqgzxgct5v9psqqszqrgukapqlycln3zg7qflvaayucr87zde8ns8xmv6r2wcpr6seans6e626dlj2xlxhwnudkvstwhv9smz6ess9ysnumhs6xywy0de6fgpqajkg6t5d9hkuscqqgpqpnkh0ghwwcdkl6k3960227d4nhpyd3e5mwmv5jsdju4tfh83kpqxwzg9nyjngnqu4tv5fggp73atvk5vmff3eta2f2vz9tfz7jlx65z848alnexmztxgt85tumzss0zsh5etpeggrplc35z8lwdyl97m2qqx685my"
                      },
                      {
                          "type": "future",
                          "id": "6890493815612895160094203812312087189345395449663723080084883036893352254202field",
                          "value": "{\n  program_id: aleo_name_service_registry_v3.aleo,\n  function_name: register,\n  arguments: [\n    1855567428766962121775891079065014092687529749879196674664926978336819369806field,\n    [\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n],\n    3601410589032411677092457044111621862970800028849492457114786804129430260029field,\n    0u128,\n    aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk\n  ]\n}"
                      }
                  ],
                  "tpk": "6925638895606706728927210929010408694485584827805980684780849325551325666850group",
                  "tcm": "5063886804062189846704902100343637895026967807966020303368241407248708773143field"
              },
              {
                  "id": "au1vhm9c4kqrrux28x855yw3ttrhfcpe8tt2u328eycd9d8s7qykq8q6lyymy",
                  "program": "ans_registrar_v3.aleo",
                  "function": "register_fld",
                  "inputs": [
                      {
                          "type": "private",
                          "id": "2413109936835612095473873127449289319886127925236694093870280185354707649922field",
                          "value": "ciphertext1qvqg7h3rd62wzdyqq0uyx0uf9tc9e89e00a0dqd9sl9e36mndejpcp278cs3l5mvnlvsqmhytp3z9nw0kttrcjkpcfq4uldxfyj42epjp9263zm4sm0yawhmjq98a4ct6udyqs67fyf78tln3evxppgn0ucsgqwdcyd"
                      },
                      {
                          "type": "private",
                          "id": "4054430502716392470361501337301482017782939902452185366321496189067430414156field",
                          "value": "ciphertext1qgqgg7z50cz9kgg7kp26cma5xcfr69mmf8yg0nptaxu6sw8kfqpwcrply3vuu84hdpkd7kcvdmzvw56guhs6wfr6sx26wq2rus8l8xacpge5swwm"
                      },
                      {
                          "type": "external_record",
                          "id": "552768722850909533074524003192605309276916889875240628573948784161143150842field"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "future",
                          "id": "311324591680934630713767857895213198461652759186517464545785967115316870007field",
                          "value": "{\n  program_id: ans_registrar_v3.aleo,\n  function_name: register_fld,\n  arguments: [\n    {\n      program_id: credits.aleo,\n      function_name: transfer_private_to_public,\n      arguments: [\n        aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,\n        2000000u64\n      ]\n    },\n    {\n      program_id: aleo_name_service_registry_v3.aleo,\n      function_name: register,\n      arguments: [\n        1855567428766962121775891079065014092687529749879196674664926978336819369806field,\n        [\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n],\n        3601410589032411677092457044111621862970800028849492457114786804129430260029field,\n        0u128,\n        aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk\n      ]\n    }\n  \n  ]\n}"
                      }
                  ],
                  "tpk": "1315787453617668025546845197512217125353118412615658445979296200245236483420group",
                  "tcm": "3200235939611944040827854218598157017513384640468128366131444470338593978612field"
              }
          ]
      }
    "#;
        let fee_authorization = r#"
        {
          "requests": [
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "ans_registrar_v3.aleo",
                  "function": "register_fld",
                  "input_ids": [
                      {
                          "type": "private",
                          "id": "2413109936835612095473873127449289319886127925236694093870280185354707649922field"
                      },
                      {
                          "type": "private",
                          "id": "4054430502716392470361501337301482017782939902452185366321496189067430414156field"
                      },
                      {
                          "type": "external_record",
                          "id": "552768722850909533074524003192605309276916889875240628573948784161143150842field"
                      }
                  ],
                  "inputs": [
                      "[\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n]",
                      "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                      "{\n  owner: aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s.private,\n  microcredits: 3360625u64.private,\n  _nonce: 6755319637731613343169990547003035554909118965409023337724706464194317148546group.public\n}"
                  ],
                  "signature": "sign17pyxvjkhpm9809v4umfxcwyh2rrur6jruhvw4a7smgl6tyfh65pc3j5xyk5p8qsnhcgcvzzf9khhk9xx9skm6cvq9vxfgu4p4rddkqtkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q69vh79t",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "615773232593459454849189940269468037301586945432863192131754508046360314863field",
                  "tcm": "3200235939611944040827854218598157017513384640468128366131444470338593978612field"
              },
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "credits.aleo",
                  "function": "transfer_private_to_public",
                  "input_ids": [
                      {
                          "type": "record",
                          "commitment": "5285743509580651613836085785444392724241985265312438718114876511545363763276field",
                          "gamma": "4939586439829843445888593228166152705538245380517674416688636202354721097782group",
                          "serial_number": "7194047904949744381042008412666492861890446076712460242169866538749591846485field",
                          "tag": "867581519213588558477924408394505423861150311985691906127848578458509312494field"
                      },
                      {
                          "type": "public",
                          "id": "4085992707787846988806173649067715950013245109798517325277546873866667738395field"
                      },
                      {
                          "type": "public",
                          "id": "4201848541613129278080414622873109941689977856740970554686587401648883512222field"
                      }
                  ],
                  "inputs": [
                      "{\n  owner: aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s.private,\n  microcredits: 3360625u64.private,\n  _nonce: 6755319637731613343169990547003035554909118965409023337724706464194317148546group.public\n}",
                      "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk",
                      "2000000u64"
                  ],
                  "signature": "sign1jxypnjrx8s4skg4yuz63zle5pk85np8duc9h9gw8w4efw7jdrqq0kfyfkkcxadf8kcuk66ffzzxr4j0xwdc58u4rt55l2qafs066kqrkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q6g9j8rs",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "2488483095255106678134679844379366627168187953291874874541516448422432108905field",
                  "tcm": "6841255139595749472275446347930895765057286243053856562446133350885139114581field"
              },
              {
                  "signer": "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                  "network": "3u16",
                  "program": "aleo_name_service_registry_v3.aleo",
                  "function": "register",
                  "input_ids": [
                      {
                          "type": "private",
                          "id": "6909035840792431568942240361481230062968957326807529302867016965729621030786field"
                      },
                      {
                          "type": "private",
                          "id": "5287731742262540871585927936299361359314418854376324674193462218381480740699field"
                      },
                      {
                          "type": "private",
                          "id": "4780648626598733420327901902796317428868812913462539543282206431218858006967field"
                      },
                      {
                          "type": "private",
                          "id": "1051666325121632181583719347829586283933225966064054779411473986216402532999field"
                      }
                  ],
                  "inputs": [
                      "[\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n]",
                      "3601410589032411677092457044111621862970800028849492457114786804129430260029field",
                      "aleo1nlwtlzch6c6qunqya5l2gf7d0t6msq34szspj763jgecuv2j7qxs7tva2s",
                      "0u128"
                  ],
                  "signature": "sign1pfaancu02esay79samvn7a6zkywasqxqvhxtqsgyjaqtvzhx3cpnfckhuglcx43q3pk8hcledxljzctmxnd43umavmvjm3kgflzyvqnkgva85tgraj5pckmdkj7p7p802lny2m8r8vyptqplvyyezl42zz67tgns6gvztgtsl33pmkkdk852fq25uqejx662yg0ylgh2yd6q6hdjq3q",
                  "sk_tag": "7448046062012620339054859002056665491562280624589429737136673399573395220929field",
                  "tvk": "2798735880835806033027265130365377522643708207060050646244265139383027748019field",
                  "tcm": "5063886804062189846704902100343637895026967807966020303368241407248708773143field"
              }
          ],
          "transitions": [
              {
                  "id": "au1vhm9c4kqrrux28x855yw3ttrhfcpe8tt2u328eycd9d8s7qykq8q6lyymy",
                  "program": "ans_registrar_v3.aleo",
                  "function": "register_fld",
                  "inputs": [
                      {
                          "type": "private",
                          "id": "2413109936835612095473873127449289319886127925236694093870280185354707649922field",
                          "value": "ciphertext1qvqg7h3rd62wzdyqq0uyx0uf9tc9e89e00a0dqd9sl9e36mndejpcp278cs3l5mvnlvsqmhytp3z9nw0kttrcjkpcfq4uldxfyj42epjp9263zm4sm0yawhmjq98a4ct6udyqs67fyf78tln3evxppgn0ucsgqwdcyd"
                      },
                      {
                          "type": "private",
                          "id": "4054430502716392470361501337301482017782939902452185366321496189067430414156field",
                          "value": "ciphertext1qgqgg7z50cz9kgg7kp26cma5xcfr69mmf8yg0nptaxu6sw8kfqpwcrply3vuu84hdpkd7kcvdmzvw56guhs6wfr6sx26wq2rus8l8xacpge5swwm"
                      },
                      {
                          "type": "external_record",
                          "id": "552768722850909533074524003192605309276916889875240628573948784161143150842field"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "future",
                          "id": "311324591680934630713767857895213198461652759186517464545785967115316870007field",
                          "value": "{\n  program_id: ans_registrar_v3.aleo,\n  function_name: register_fld,\n  arguments: [\n    {\n      program_id: credits.aleo,\n      function_name: transfer_private_to_public,\n      arguments: [\n        aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,\n        2000000u64\n      ]\n    },\n    {\n      program_id: aleo_name_service_registry_v3.aleo,\n      function_name: register,\n      arguments: [\n        1855567428766962121775891079065014092687529749879196674664926978336819369806field,\n        [\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n],\n        3601410589032411677092457044111621862970800028849492457114786804129430260029field,\n        0u128,\n        aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk\n      ]\n    }\n  \n  ]\n}"
                      }
                  ],
                  "tpk": "1315787453617668025546845197512217125353118412615658445979296200245236483420group",
                  "tcm": "3200235939611944040827854218598157017513384640468128366131444470338593978612field"
              },
              {
                  "id": "au1gk2xraje9m7hp4l073m4nsyc9hp22l9jhcy0whyxdd5ancwxcq8qftq74d",
                  "program": "credits.aleo",
                  "function": "transfer_private_to_public",
                  "inputs": [
                      {
                          "type": "record",
                          "id": "7194047904949744381042008412666492861890446076712460242169866538749591846485field",
                          "tag": "867581519213588558477924408394505423861150311985691906127848578458509312494field"
                      },
                      {
                          "type": "public",
                          "id": "4085992707787846988806173649067715950013245109798517325277546873866667738395field",
                          "value": "aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk"
                      },
                      {
                          "type": "public",
                          "id": "4201848541613129278080414622873109941689977856740970554686587401648883512222field",
                          "value": "2000000u64"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "record",
                          "id": "5038802135377020518179838167452708663270964125145877803104694628649134935117field",
                          "checksum": "6220691502790345109534922685261605533680725175185198318951675108525455348773field",
                          "value": "record1qyqsp4t0wv7ap75s5nkxc3z5jan2jhepf9fhsqkqeq6pr6tdmv8c5vqwqyxx66trwfhkxun9v35hguerqqpqzqx56z7l7l3pnj98l9zlyrqf22gknext47jvj4vlsr20ljpsgzguq45vxaswk48gulgpqjuql6n35p0uazdqywu8hzxyaa5vcrc6f8dq7ncxk9j"
                      },
                      {
                          "type": "future",
                          "id": "3237012691650891316823433605543950000833525309701820848507373470024687526235field",
                          "value": "{\n  program_id: credits.aleo,\n  function_name: transfer_private_to_public,\n  arguments: [\n    aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk,\n    2000000u64\n  ]\n}"
                      }
                  ],
                  "tpk": "2076819040045510863228558773346133675190800676730155837100903172584756663871group",
                  "tcm": "6841255139595749472275446347930895765057286243053856562446133350885139114581field"
              },
              {
                  "id": "au1jqtgn3j0x2cr2pu5g3sdx8s83anfvu86jrmc48yl2j7udwvn5qpsuafh3q",
                  "program": "aleo_name_service_registry_v3.aleo",
                  "function": "register",
                  "inputs": [
                      {
                          "type": "private",
                          "id": "6909035840792431568942240361481230062968957326807529302867016965729621030786field",
                          "value": "ciphertext1qvqxj92g2h469hx5rx50anqpkpula0j4hdl9jkkctq5pxkcr7gnwyyd5yf7accwqjcxdtc3ggyrw5a99au8tm33eqqspx76q2le9kmp2p9rfq9u2ul55d65px9j2k6743hlp96hhjhxk9qrvfuffxhpw0q8sxmk0qu9"
                      },
                      {
                          "type": "private",
                          "id": "5287731742262540871585927936299361359314418854376324674193462218381480740699field",
                          "value": "ciphertext1qgqzjvdetjjuaz7qpgkthn9p7xck0y82shfkye756y4tezzta8sj7p8tjgtcnpsrt8hkz3ggh2xlpm9t5fhp7qerr2mwlj4af0p00hphzqkmeyrl"
                      },
                      {
                          "type": "private",
                          "id": "4780648626598733420327901902796317428868812913462539543282206431218858006967field",
                          "value": "ciphertext1qgq0dgv5z90alwz5plt88rux3x729249n38efw90ta6cczl2r6v0yrhfc4z468z3ttffqldvmfg8mrju70duegrcm0zk6slff8lezm3tpvx2udmh"
                      },
                      {
                          "type": "private",
                          "id": "1051666325121632181583719347829586283933225966064054779411473986216402532999field",
                          "value": "ciphertext1qyqyu7kac2u9cfc5rtl7ukp2l49lenwksfta0hr9d5jpr2efwj66uqgzmuj97"
                      }
                  ],
                  "outputs": [
                      {
                          "type": "record",
                          "id": "6259488067471386787837075778944647574465061092993325020240541141015439965652field",
                          "checksum": "5164963297651619656029586786429889762272560621538667310903856175441498615580field",
                          "value": "record1qyqspn4mjrcuym8g3e3mfc4s8xk6mplcrnj00n7hmzejwacc8sw3v7gpqgzxgct5v9psqqszqrgukapqlycln3zg7qflvaayucr87zde8ns8xmv6r2wcpr6seans6e626dlj2xlxhwnudkvstwhv9smz6ess9ysnumhs6xywy0de6fgpqajkg6t5d9hkuscqqgpqpnkh0ghwwcdkl6k3960227d4nhpyd3e5mwmv5jsdju4tfh83kpqxwzg9nyjngnqu4tv5fggp73atvk5vmff3eta2f2vz9tfz7jlx65z848alnexmztxgt85tumzss0zsh5etpeggrplc35z8lwdyl97m2qqx685my"
                      },
                      {
                          "type": "future",
                          "id": "6890493815612895160094203812312087189345395449663723080084883036893352254202field",
                          "value": "{\n  program_id: aleo_name_service_registry_v3.aleo,\n  function_name: register,\n  arguments: [\n    1855567428766962121775891079065014092687529749879196674664926978336819369806field,\n    [\n  121424957042547u128,\n  0u128,\n  0u128,\n  0u128\n],\n    3601410589032411677092457044111621862970800028849492457114786804129430260029field,\n    0u128,\n    aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk\n  ]\n}"
                      }
                  ],
                  "tpk": "6925638895606706728927210929010408694485584827805980684780849325551325666850group",
                  "tcm": "5063886804062189846704902100343637895026967807966020303368241407248708773143field"
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

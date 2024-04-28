pub mod commands;
pub use commands::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
        let query = Some("https://mainnetv0.sotertech.io");

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
    fn cost_execute_fee() {
        let program_id = "inscription_v1.aleo";
        let function = "mint_private";

        let inputs = vec!["{tick:[108u8, 101u8, 111u8, 115u8], amt: 1u128}".to_string()];
        let query = Some("https://mainnetv0.sotertech.io");
        let base_fee = commands::execution_cost(program_id, function, inputs, query);
        println!("cost_execute_fee transaction: {}", base_fee.unwrap());
    }

    #[test]
    fn cost_deployment_fee() {
        let program_code = r#"
        import credits.aleo;
import aleo_name_service_registry_v3.aleo;

program ans_registrar_v3.aleo;

mapping general_settings:
    key as u8.public;
    value as u128.public;

closure calc_price:
    input r0 as [u128; 4u32];
    gt r0[0u32] 4294967295u128 into r1;
    gt r0[0u32] 16777215u128 into r2;
    gt r0[0u32] 65535u128 into r3;
    gt r0[0u32] 255u128 into r4;
    ternary r4 250000000u64 1250000000u64 into r5;
    ternary r3 50000000u64 r5 into r6;
    ternary r2 10000000u64 r6 into r7;
    ternary r1 2000000u64 r7 into r8;
    output r8 as u64;

function initialize:
    hash.bhp256 self.caller into r0 as u128;
    async initialize r0 into r1;
    output r1 as ans_registrar_v3.aleo/initialize.future;

finalize initialize:
    input r0 as u128.public;
    set r0 into general_settings[0u8];
    set 1u128 into general_settings[1u8];

function set_minting_flag:
    input r0 as u128.private;
    hash.bhp256 self.caller into r1 as u128;
    async set_minting_flag r1 r0 into r2;
    output r2 as ans_registrar_v3.aleo/set_minting_flag.future;

finalize set_minting_flag:
    input r0 as u128.public;
    input r1 as u128.public;
    get general_settings[0u8] into r2;
    assert.eq r0 r2 ;
    set r1 into general_settings[1u8];

function set_admin:
    input r0 as address.private;
    hash.bhp256 self.caller into r1 as u128;
    hash.bhp256 r0 into r2 as u128;
    async set_admin r1 r2 into r3;
    output r3 as ans_registrar_v3.aleo/set_admin.future;

finalize set_admin:
    input r0 as u128.public;
    input r1 as u128.public;
    get general_settings[0u8] into r2;
    assert.eq r0 r2 ;
    set r1 into general_settings[7u8];

function register_fld:
    input r0 as [u128; 4u32].private;
    input r1 as address.private;
    input r2 as credits.aleo/credits.record;
    call calc_price r0 into r3;
    call credits.aleo/transfer_private_to_public r2 aleo1xx4hvsfpdytut05cc2g0au9x84uw6qrs66v32ufwde3d8ury0vgqqupmzk r3 into r4 r5;
    call aleo_name_service_registry_v3.aleo/register r0 3601410589032411677092457044111621862970800028849492457114786804129430260029field r1 0u128 into r6 r7;
    async register_fld r5 r7 into r8;
    output r8 as ans_registrar_v3.aleo/register_fld.future;

finalize register_fld:
    input r0 as credits.aleo/transfer_private_to_public.future;
    input r1 as aleo_name_service_registry_v3.aleo/register.future;
    await r0;
    await r1;
    get general_settings[1u8] into r2;
    assert.eq r2 1u128 ;

function claim:
    input r0 as u64.private;
    input r1 as address.private;
    hash.bhp256 self.caller into r2 as u128;
    call credits.aleo/transfer_public_to_private r1 r0 into r3 r4;
    async claim r4 r2 into r5;
    output r5 as ans_registrar_v3.aleo/claim.future;

finalize claim:
    input r0 as credits.aleo/transfer_public_to_private.future;
    input r1 as u128.public;
    await r0;
    get general_settings[0u8] into r2;
    assert.eq r1 r2 ;

        "#;

        let import_name_1 = "credits.aleo";

        let import_code_1 = r#"
        program credits.aleo;

mapping committee:
    key as address.public;
    value as committee_state.public;

struct committee_state:
    microcredits as u64;
    is_open as boolean;

mapping bonded:
    key as address.public;
    value as bond_state.public;

struct bond_state:
    validator as address;
    microcredits as u64;

mapping unbonding:
    key as address.public;
    value as unbond_state.public;

struct unbond_state:
    microcredits as u64;
    height as u32;

mapping account:
    key as address.public;
    value as u64.public;

record credits:
    owner as address.private;
    microcredits as u64.private;

function bond_public:
    input r0 as address.public;
    input r1 as u64.public;
    gte r1 1000000u64 into r2;
    assert.eq r2 true ;
    async bond_public self.caller r0 r1 into r3;
    output r3 as credits.aleo/bond_public.future;

finalize bond_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    is.eq r0 r1 into r3;
    branch.eq r3 true to bond_validator;
    branch.eq r3 false to bond_delegator;
    position bond_validator;
    cast 0u64 true into r4 as committee_state;
    get.or_use committee[r0] r4 into r5;
    assert.eq r5.is_open true ;
    add r5.microcredits r2 into r6;
    cast r6 r5.is_open into r7 as committee_state;
    cast r1 0u64 into r8 as bond_state;
    get.or_use bonded[r0] r8 into r9;
    assert.eq r9.validator r1 ;
    add r9.microcredits r2 into r10;
    gte r10 1000000000000u64 into r11;
    assert.eq r11 true ;
    cast r1 r10 into r12 as bond_state;
    get account[r0] into r13;
    sub r13 r2 into r14;
    set r7 into committee[r0];
    set r12 into bonded[r0];
    set r14 into account[r0];
    branch.eq true true to end;
    position bond_delegator;
    contains committee[r0] into r15;
    assert.eq r15 false ;
    get committee[r1] into r16;
    assert.eq r16.is_open true ;
    add r16.microcredits r2 into r17;
    cast r17 r16.is_open into r18 as committee_state;
    cast r1 0u64 into r19 as bond_state;
    get.or_use bonded[r0] r19 into r20;
    assert.eq r20.validator r1 ;
    add r20.microcredits r2 into r21;
    gte r21 10000000u64 into r22;
    assert.eq r22 true ;
    cast r1 r21 into r23 as bond_state;
    get account[r0] into r24;
    sub r24 r2 into r25;
    set r18 into committee[r1];
    set r23 into bonded[r0];
    set r25 into account[r0];
    position end;

function unbond_public:
    input r0 as u64.public;
    async unbond_public self.caller r0 into r1;
    output r1 as credits.aleo/unbond_public.future;

finalize unbond_public:
    input r0 as address.public;
    input r1 as u64.public;
    cast 0u64 0u32 into r2 as unbond_state;
    get.or_use unbonding[r0] r2 into r3;
    add block.height 360u32 into r4;
    contains committee[r0] into r5;
    branch.eq r5 true to unbond_validator;
    branch.eq r5 false to unbond_delegator;
    position unbond_validator;
    get committee[r0] into r6;
    sub r6.microcredits r1 into r7;
    get bonded[r0] into r8;
    assert.eq r8.validator r0 ;
    sub r8.microcredits r1 into r9;
    gte r9 1000000000000u64 into r10;
    branch.eq r10 true to decrement_validator;
    branch.eq r10 false to remove_validator;
    position decrement_validator;
    cast r7 r6.is_open into r11 as committee_state;
    set r11 into committee[r0];
    cast r0 r9 into r12 as bond_state;
    set r12 into bonded[r0];
    add r3.microcredits r1 into r13;
    cast r13 r4 into r14 as unbond_state;
    set r14 into unbonding[r0];
    branch.eq true true to end;
    position remove_validator;
    assert.eq r6.microcredits r8.microcredits ;
    remove committee[r0];
    remove bonded[r0];
    add r3.microcredits r8.microcredits into r15;
    cast r15 r4 into r16 as unbond_state;
    set r16 into unbonding[r0];
    branch.eq true true to end;
    position unbond_delegator;
    get bonded[r0] into r17;
    sub r17.microcredits r1 into r18;
    gte r18 10000000u64 into r19;
    branch.eq r19 true to decrement_delegator;
    branch.eq r19 false to remove_delegator;
    position decrement_delegator;
    get committee[r17.validator] into r20;
    sub r20.microcredits r1 into r21;
    cast r21 r20.is_open into r22 as committee_state;
    set r22 into committee[r17.validator];
    cast r17.validator r18 into r23 as bond_state;
    set r23 into bonded[r0];
    add r3.microcredits r1 into r24;
    cast r24 r4 into r25 as unbond_state;
    set r25 into unbonding[r0];
    branch.eq true true to end;
    position remove_delegator;
    get committee[r17.validator] into r26;
    sub r26.microcredits r17.microcredits into r27;
    cast r27 r26.is_open into r28 as committee_state;
    set r28 into committee[r17.validator];
    remove bonded[r0];
    add r3.microcredits r17.microcredits into r29;
    cast r29 r4 into r30 as unbond_state;
    set r30 into unbonding[r0];
    position end;

function unbond_delegator_as_validator:
    input r0 as address.public;
    async unbond_delegator_as_validator self.caller r0 into r1;
    output r1 as credits.aleo/unbond_delegator_as_validator.future;

finalize unbond_delegator_as_validator:
    input r0 as address.public;
    input r1 as address.public;
    get committee[r0] into r2;
    assert.eq r2.is_open false ;
    contains committee[r1] into r3;
    assert.eq r3 false ;
    get bonded[r1] into r4;
    assert.eq r4.validator r0 ;
    sub r2.microcredits r4.microcredits into r5;
    cast r5 r2.is_open into r6 as committee_state;
    cast 0u64 0u32 into r7 as unbond_state;
    get.or_use unbonding[r1] r7 into r8;
    add r8.microcredits r4.microcredits into r9;
    add block.height 360u32 into r10;
    cast r9 r10 into r11 as unbond_state;
    set r6 into committee[r0];
    remove bonded[r1];
    set r11 into unbonding[r1];

function claim_unbond_public:
    async claim_unbond_public self.caller into r0;
    output r0 as credits.aleo/claim_unbond_public.future;

finalize claim_unbond_public:
    input r0 as address.public;
    get unbonding[r0] into r1;
    gte block.height r1.height into r2;
    assert.eq r2 true ;
    get.or_use account[r0] 0u64 into r3;
    add r1.microcredits r3 into r4;
    set r4 into account[r0];
    remove unbonding[r0];

function set_validator_state:
    input r0 as boolean.public;
    async set_validator_state self.caller r0 into r1;
    output r1 as credits.aleo/set_validator_state.future;

finalize set_validator_state:
    input r0 as address.public;
    input r1 as boolean.public;
    get committee[r0] into r2;
    cast r2.microcredits r1 into r3 as committee_state;
    set r3 into committee[r0];

function transfer_public:
    input r0 as address.public;
    input r1 as u64.public;
    async transfer_public self.caller r0 r1 into r2;
    output r2 as credits.aleo/transfer_public.future;

finalize transfer_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    get.or_use account[r0] 0u64 into r3;
    sub r3 r2 into r4;
    set r4 into account[r0];
    get.or_use account[r1] 0u64 into r5;
    add r5 r2 into r6;
    set r6 into account[r1];

function transfer_private:
    input r0 as credits.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.microcredits r2 into r3;
    cast r1 r2 into r4 as credits.record;
    cast r0.owner r3 into r5 as credits.record;
    output r4 as credits.record;
    output r5 as credits.record;

function transfer_private_to_public:
    input r0 as credits.record;
    input r1 as address.public;
    input r2 as u64.public;
    sub r0.microcredits r2 into r3;
    cast r0.owner r3 into r4 as credits.record;
    async transfer_private_to_public r1 r2 into r5;
    output r4 as credits.record;
    output r5 as credits.aleo/transfer_private_to_public.future;

finalize transfer_private_to_public:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    add r1 r2 into r3;
    set r3 into account[r0];

function transfer_public_to_private:
    input r0 as address.private;
    input r1 as u64.public;
    cast r0 r1 into r2 as credits.record;
    async transfer_public_to_private self.caller r1 into r3;
    output r2 as credits.record;
    output r3 as credits.aleo/transfer_public_to_private.future;

finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];

function join:
    input r0 as credits.record;
    input r1 as credits.record;
    add r0.microcredits r1.microcredits into r2;
    cast r0.owner r2 into r3 as credits.record;
    output r3 as credits.record;

function split:
    input r0 as credits.record;
    input r1 as u64.private;
    sub r0.microcredits r1 into r2;
    sub r2 10000u64 into r3;
    cast r0.owner r1 into r4 as credits.record;
    cast r0.owner r3 into r5 as credits.record;
    output r4 as credits.record;
    output r5 as credits.record;

function fee_private:
    input r0 as credits.record;
    input r1 as u64.public;
    input r2 as field.public;
    assert.neq r1 0u64 ;
    assert.neq r2 0field ;
    sub r0.microcredits r1 into r3;
    cast r0.owner r3 into r4 as credits.record;
    output r4 as credits.record;

function fee_public:
    input r0 as u64.public;
    input r1 as field.public;
    assert.neq r0 0u64 ;
    assert.neq r1 0field ;
    async fee_public self.caller r0 into r2;
    output r2 as credits.aleo/fee_public.future;

finalize fee_public:
    input r0 as address.public;
    input r1 as u64.public;
    get account[r0] into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];

        "#;

        let import_name_2 = "aleo_name_service_registry_v3.aleo";

        let import_code_2 = r#"
        import credits.aleo;

program aleo_name_service_registry_v3.aleo;

struct BaseURI:
    data0 as u128;
    data1 as u128;
    data2 as u128;
    data3 as u128;

struct ResolverIndex:
    name as field;
    category as u128;
    version as u64;

struct Name:
    name as [u128; 4u32];
    parent as field;

struct NameStruct:
    name as [u128; 4u32];
    parent as field;
    resolver as u128;

struct AddressPair:
    nft_owner as address;
    nft_operator as address;

struct NameAddressPair:
    name_hash as field;
    nft_owner as address;

record NFT:
    owner as address.private;
    data as field.private;
    edition as scalar.private;

record NFT_ownership:
    owner as address.private;
    nft_owner as address.private;
    data as field.private;
    edition as scalar.private;

mapping nft_owners:
    key as field.public;
    value as address.public;

mapping names:
    key as field.public;
    value as NameStruct.public;

mapping primary_names:
    key as address.public;
    value as field.public;

mapping resolvers:
    key as ResolverIndex.public;
    value as [u128; 8u32].public;

mapping name_versions:
    key as field.public;
    value as u64.public;

mapping approved_for_all:
    key as AddressPair.public;
    value as boolean.public;

mapping approved_for_nft:
    key as NameAddressPair.public;
    value as address.public;

mapping general_settings:
    key as u8.public;
    value as u128.public;

mapping toggle_settings:
    key as u8.public;
    value as u32.public;

closure validate_name:
    input r0 as [u128; 4u32];
    assert.neq r0[0u32] 0u128 ;
    mul 0u8 8u8 into r1;
    shr r0[0u32] r1 into r2;
    and r2 255u128 into r3;
    is.eq r3 0u128 into r4;
    is.eq r3 45u128 into r5;
    or r4 r5 into r6;
    is.eq r3 95u128 into r7;
    or r6 r7 into r8;
    gte r3 48u128 into r9;
    lte r3 57u128 into r10;
    and r9 r10 into r11;
    or r8 r11 into r12;
    gte r3 97u128 into r13;
    lte r3 122u128 into r14;
    and r13 r14 into r15;
    or r12 r15 into r16;
    not r16 into r17;
    mul 1u8 8u8 into r18;
    shr r0[0u32] r18 into r19;
    and r19 255u128 into r20;
    is.eq r20 0u128 into r21;
    is.eq r20 45u128 into r22;
    or r21 r22 into r23;
    is.eq r20 95u128 into r24;
    or r23 r24 into r25;
    gte r20 48u128 into r26;
    lte r20 57u128 into r27;
    and r26 r27 into r28;
    or r25 r28 into r29;
    gte r20 97u128 into r30;
    lte r20 122u128 into r31;
    and r30 r31 into r32;
    or r29 r32 into r33;
    not r33 into r34;
    mul 2u8 8u8 into r35;
    shr r0[0u32] r35 into r36;
    and r36 255u128 into r37;
    is.eq r37 0u128 into r38;
    is.eq r37 45u128 into r39;
    or r38 r39 into r40;
    is.eq r37 95u128 into r41;
    or r40 r41 into r42;
    gte r37 48u128 into r43;
    lte r37 57u128 into r44;
    and r43 r44 into r45;
    or r42 r45 into r46;
    gte r37 97u128 into r47;
    lte r37 122u128 into r48;
    and r47 r48 into r49;
    or r46 r49 into r50;
    not r50 into r51;
    mul 3u8 8u8 into r52;
    shr r0[0u32] r52 into r53;
    and r53 255u128 into r54;
    is.eq r54 0u128 into r55;
    is.eq r54 45u128 into r56;
    or r55 r56 into r57;
    is.eq r54 95u128 into r58;
    or r57 r58 into r59;
    gte r54 48u128 into r60;
    lte r54 57u128 into r61;
    and r60 r61 into r62;
    or r59 r62 into r63;
    gte r54 97u128 into r64;
    lte r54 122u128 into r65;
    and r64 r65 into r66;
    or r63 r66 into r67;
    not r67 into r68;
    mul 4u8 8u8 into r69;
    shr r0[0u32] r69 into r70;
    and r70 255u128 into r71;
    is.eq r71 0u128 into r72;
    is.eq r71 45u128 into r73;
    or r72 r73 into r74;
    is.eq r71 95u128 into r75;
    or r74 r75 into r76;
    gte r71 48u128 into r77;
    lte r71 57u128 into r78;
    and r77 r78 into r79;
    or r76 r79 into r80;
    gte r71 97u128 into r81;
    lte r71 122u128 into r82;
    and r81 r82 into r83;
    or r80 r83 into r84;
    not r84 into r85;
    mul 5u8 8u8 into r86;
    shr r0[0u32] r86 into r87;
    and r87 255u128 into r88;
    is.eq r88 0u128 into r89;
    is.eq r88 45u128 into r90;
    or r89 r90 into r91;
    is.eq r88 95u128 into r92;
    or r91 r92 into r93;
    gte r88 48u128 into r94;
    lte r88 57u128 into r95;
    and r94 r95 into r96;
    or r93 r96 into r97;
    gte r88 97u128 into r98;
    lte r88 122u128 into r99;
    and r98 r99 into r100;
    or r97 r100 into r101;
    not r101 into r102;
    mul 6u8 8u8 into r103;
    shr r0[0u32] r103 into r104;
    and r104 255u128 into r105;
    is.eq r105 0u128 into r106;
    is.eq r105 45u128 into r107;
    or r106 r107 into r108;
    is.eq r105 95u128 into r109;
    or r108 r109 into r110;
    gte r105 48u128 into r111;
    lte r105 57u128 into r112;
    and r111 r112 into r113;
    or r110 r113 into r114;
    gte r105 97u128 into r115;
    lte r105 122u128 into r116;
    and r115 r116 into r117;
    or r114 r117 into r118;
    not r118 into r119;
    mul 7u8 8u8 into r120;
    shr r0[0u32] r120 into r121;
    and r121 255u128 into r122;
    is.eq r122 0u128 into r123;
    is.eq r122 45u128 into r124;
    or r123 r124 into r125;
    is.eq r122 95u128 into r126;
    or r125 r126 into r127;
    gte r122 48u128 into r128;
    lte r122 57u128 into r129;
    and r128 r129 into r130;
    or r127 r130 into r131;
    gte r122 97u128 into r132;
    lte r122 122u128 into r133;
    and r132 r133 into r134;
    or r131 r134 into r135;
    not r135 into r136;
    mul 8u8 8u8 into r137;
    shr r0[0u32] r137 into r138;
    and r138 255u128 into r139;
    is.eq r139 0u128 into r140;
    is.eq r139 45u128 into r141;
    or r140 r141 into r142;
    is.eq r139 95u128 into r143;
    or r142 r143 into r144;
    gte r139 48u128 into r145;
    lte r139 57u128 into r146;
    and r145 r146 into r147;
    or r144 r147 into r148;
    gte r139 97u128 into r149;
    lte r139 122u128 into r150;
    and r149 r150 into r151;
    or r148 r151 into r152;
    not r152 into r153;
    mul 9u8 8u8 into r154;
    shr r0[0u32] r154 into r155;
    and r155 255u128 into r156;
    is.eq r156 0u128 into r157;
    is.eq r156 45u128 into r158;
    or r157 r158 into r159;
    is.eq r156 95u128 into r160;
    or r159 r160 into r161;
    gte r156 48u128 into r162;
    lte r156 57u128 into r163;
    and r162 r163 into r164;
    or r161 r164 into r165;
    gte r156 97u128 into r166;
    lte r156 122u128 into r167;
    and r166 r167 into r168;
    or r165 r168 into r169;
    not r169 into r170;
    mul 10u8 8u8 into r171;
    shr r0[0u32] r171 into r172;
    and r172 255u128 into r173;
    is.eq r173 0u128 into r174;
    is.eq r173 45u128 into r175;
    or r174 r175 into r176;
    is.eq r173 95u128 into r177;
    or r176 r177 into r178;
    gte r173 48u128 into r179;
    lte r173 57u128 into r180;
    and r179 r180 into r181;
    or r178 r181 into r182;
    gte r173 97u128 into r183;
    lte r173 122u128 into r184;
    and r183 r184 into r185;
    or r182 r185 into r186;
    not r186 into r187;
    mul 11u8 8u8 into r188;
    shr r0[0u32] r188 into r189;
    and r189 255u128 into r190;
    is.eq r190 0u128 into r191;
    is.eq r190 45u128 into r192;
    or r191 r192 into r193;
    is.eq r190 95u128 into r194;
    or r193 r194 into r195;
    gte r190 48u128 into r196;
    lte r190 57u128 into r197;
    and r196 r197 into r198;
    or r195 r198 into r199;
    gte r190 97u128 into r200;
    lte r190 122u128 into r201;
    and r200 r201 into r202;
    or r199 r202 into r203;
    not r203 into r204;
    mul 12u8 8u8 into r205;
    shr r0[0u32] r205 into r206;
    and r206 255u128 into r207;
    is.eq r207 0u128 into r208;
    is.eq r207 45u128 into r209;
    or r208 r209 into r210;
    is.eq r207 95u128 into r211;
    or r210 r211 into r212;
    gte r207 48u128 into r213;
    lte r207 57u128 into r214;
    and r213 r214 into r215;
    or r212 r215 into r216;
    gte r207 97u128 into r217;
    lte r207 122u128 into r218;
    and r217 r218 into r219;
    or r216 r219 into r220;
    not r220 into r221;
    mul 13u8 8u8 into r222;
    shr r0[0u32] r222 into r223;
    and r223 255u128 into r224;
    is.eq r224 0u128 into r225;
    is.eq r224 45u128 into r226;
    or r225 r226 into r227;
    is.eq r224 95u128 into r228;
    or r227 r228 into r229;
    gte r224 48u128 into r230;
    lte r224 57u128 into r231;
    and r230 r231 into r232;
    or r229 r232 into r233;
    gte r224 97u128 into r234;
    lte r224 122u128 into r235;
    and r234 r235 into r236;
    or r233 r236 into r237;
    not r237 into r238;
    mul 14u8 8u8 into r239;
    shr r0[0u32] r239 into r240;
    and r240 255u128 into r241;
    is.eq r241 0u128 into r242;
    is.eq r241 45u128 into r243;
    or r242 r243 into r244;
    is.eq r241 95u128 into r245;
    or r244 r245 into r246;
    gte r241 48u128 into r247;
    lte r241 57u128 into r248;
    and r247 r248 into r249;
    or r246 r249 into r250;
    gte r241 97u128 into r251;
    lte r241 122u128 into r252;
    and r251 r252 into r253;
    or r250 r253 into r254;
    not r254 into r255;
    mul 15u8 8u8 into r256;
    shr r0[0u32] r256 into r257;
    and r257 255u128 into r258;
    is.eq r258 0u128 into r259;
    is.eq r258 45u128 into r260;
    or r259 r260 into r261;
    is.eq r258 95u128 into r262;
    or r261 r262 into r263;
    gte r258 48u128 into r264;
    lte r258 57u128 into r265;
    and r264 r265 into r266;
    or r263 r266 into r267;
    gte r258 97u128 into r268;
    lte r258 122u128 into r269;
    and r268 r269 into r270;
    or r267 r270 into r271;
    not r271 into r272;
    ternary r272 false true into r273;
    ternary r255 false r273 into r274;
    ternary r238 false r274 into r275;
    ternary r221 false r275 into r276;
    ternary r204 false r276 into r277;
    ternary r187 false r277 into r278;
    ternary r170 false r278 into r279;
    ternary r153 false r279 into r280;
    ternary r136 false r280 into r281;
    ternary r119 false r281 into r282;
    ternary r102 false r282 into r283;
    ternary r85 false r283 into r284;
    ternary r68 false r284 into r285;
    ternary r51 false r285 into r286;
    ternary r34 false r286 into r287;
    ternary r17 false r287 into r288;
    and true r288 into r289;
    mul 0u8 8u8 into r290;
    shr r0[1u32] r290 into r291;
    and r291 255u128 into r292;
    is.eq r292 0u128 into r293;
    is.eq r292 45u128 into r294;
    or r293 r294 into r295;
    is.eq r292 95u128 into r296;
    or r295 r296 into r297;
    gte r292 48u128 into r298;
    lte r292 57u128 into r299;
    and r298 r299 into r300;
    or r297 r300 into r301;
    gte r292 97u128 into r302;
    lte r292 122u128 into r303;
    and r302 r303 into r304;
    or r301 r304 into r305;
    not r305 into r306;
    mul 1u8 8u8 into r307;
    shr r0[1u32] r307 into r308;
    and r308 255u128 into r309;
    is.eq r309 0u128 into r310;
    is.eq r309 45u128 into r311;
    or r310 r311 into r312;
    is.eq r309 95u128 into r313;
    or r312 r313 into r314;
    gte r309 48u128 into r315;
    lte r309 57u128 into r316;
    and r315 r316 into r317;
    or r314 r317 into r318;
    gte r309 97u128 into r319;
    lte r309 122u128 into r320;
    and r319 r320 into r321;
    or r318 r321 into r322;
    not r322 into r323;
    mul 2u8 8u8 into r324;
    shr r0[1u32] r324 into r325;
    and r325 255u128 into r326;
    is.eq r326 0u128 into r327;
    is.eq r326 45u128 into r328;
    or r327 r328 into r329;
    is.eq r326 95u128 into r330;
    or r329 r330 into r331;
    gte r326 48u128 into r332;
    lte r326 57u128 into r333;
    and r332 r333 into r334;
    or r331 r334 into r335;
    gte r326 97u128 into r336;
    lte r326 122u128 into r337;
    and r336 r337 into r338;
    or r335 r338 into r339;
    not r339 into r340;
    mul 3u8 8u8 into r341;
    shr r0[1u32] r341 into r342;
    and r342 255u128 into r343;
    is.eq r343 0u128 into r344;
    is.eq r343 45u128 into r345;
    or r344 r345 into r346;
    is.eq r343 95u128 into r347;
    or r346 r347 into r348;
    gte r343 48u128 into r349;
    lte r343 57u128 into r350;
    and r349 r350 into r351;
    or r348 r351 into r352;
    gte r343 97u128 into r353;
    lte r343 122u128 into r354;
    and r353 r354 into r355;
    or r352 r355 into r356;
    not r356 into r357;
    mul 4u8 8u8 into r358;
    shr r0[1u32] r358 into r359;
    and r359 255u128 into r360;
    is.eq r360 0u128 into r361;
    is.eq r360 45u128 into r362;
    or r361 r362 into r363;
    is.eq r360 95u128 into r364;
    or r363 r364 into r365;
    gte r360 48u128 into r366;
    lte r360 57u128 into r367;
    and r366 r367 into r368;
    or r365 r368 into r369;
    gte r360 97u128 into r370;
    lte r360 122u128 into r371;
    and r370 r371 into r372;
    or r369 r372 into r373;
    not r373 into r374;
    mul 5u8 8u8 into r375;
    shr r0[1u32] r375 into r376;
    and r376 255u128 into r377;
    is.eq r377 0u128 into r378;
    is.eq r377 45u128 into r379;
    or r378 r379 into r380;
    is.eq r377 95u128 into r381;
    or r380 r381 into r382;
    gte r377 48u128 into r383;
    lte r377 57u128 into r384;
    and r383 r384 into r385;
    or r382 r385 into r386;
    gte r377 97u128 into r387;
    lte r377 122u128 into r388;
    and r387 r388 into r389;
    or r386 r389 into r390;
    not r390 into r391;
    mul 6u8 8u8 into r392;
    shr r0[1u32] r392 into r393;
    and r393 255u128 into r394;
    is.eq r394 0u128 into r395;
    is.eq r394 45u128 into r396;
    or r395 r396 into r397;
    is.eq r394 95u128 into r398;
    or r397 r398 into r399;
    gte r394 48u128 into r400;
    lte r394 57u128 into r401;
    and r400 r401 into r402;
    or r399 r402 into r403;
    gte r394 97u128 into r404;
    lte r394 122u128 into r405;
    and r404 r405 into r406;
    or r403 r406 into r407;
    not r407 into r408;
    mul 7u8 8u8 into r409;
    shr r0[1u32] r409 into r410;
    and r410 255u128 into r411;
    is.eq r411 0u128 into r412;
    is.eq r411 45u128 into r413;
    or r412 r413 into r414;
    is.eq r411 95u128 into r415;
    or r414 r415 into r416;
    gte r411 48u128 into r417;
    lte r411 57u128 into r418;
    and r417 r418 into r419;
    or r416 r419 into r420;
    gte r411 97u128 into r421;
    lte r411 122u128 into r422;
    and r421 r422 into r423;
    or r420 r423 into r424;
    not r424 into r425;
    mul 8u8 8u8 into r426;
    shr r0[1u32] r426 into r427;
    and r427 255u128 into r428;
    is.eq r428 0u128 into r429;
    is.eq r428 45u128 into r430;
    or r429 r430 into r431;
    is.eq r428 95u128 into r432;
    or r431 r432 into r433;
    gte r428 48u128 into r434;
    lte r428 57u128 into r435;
    and r434 r435 into r436;
    or r433 r436 into r437;
    gte r428 97u128 into r438;
    lte r428 122u128 into r439;
    and r438 r439 into r440;
    or r437 r440 into r441;
    not r441 into r442;
    mul 9u8 8u8 into r443;
    shr r0[1u32] r443 into r444;
    and r444 255u128 into r445;
    is.eq r445 0u128 into r446;
    is.eq r445 45u128 into r447;
    or r446 r447 into r448;
    is.eq r445 95u128 into r449;
    or r448 r449 into r450;
    gte r445 48u128 into r451;
    lte r445 57u128 into r452;
    and r451 r452 into r453;
    or r450 r453 into r454;
    gte r445 97u128 into r455;
    lte r445 122u128 into r456;
    and r455 r456 into r457;
    or r454 r457 into r458;
    not r458 into r459;
    mul 10u8 8u8 into r460;
    shr r0[1u32] r460 into r461;
    and r461 255u128 into r462;
    is.eq r462 0u128 into r463;
    is.eq r462 45u128 into r464;
    or r463 r464 into r465;
    is.eq r462 95u128 into r466;
    or r465 r466 into r467;
    gte r462 48u128 into r468;
    lte r462 57u128 into r469;
    and r468 r469 into r470;
    or r467 r470 into r471;
    gte r462 97u128 into r472;
    lte r462 122u128 into r473;
    and r472 r473 into r474;
    or r471 r474 into r475;
    not r475 into r476;
    mul 11u8 8u8 into r477;
    shr r0[1u32] r477 into r478;
    and r478 255u128 into r479;
    is.eq r479 0u128 into r480;
    is.eq r479 45u128 into r481;
    or r480 r481 into r482;
    is.eq r479 95u128 into r483;
    or r482 r483 into r484;
    gte r479 48u128 into r485;
    lte r479 57u128 into r486;
    and r485 r486 into r487;
    or r484 r487 into r488;
    gte r479 97u128 into r489;
    lte r479 122u128 into r490;
    and r489 r490 into r491;
    or r488 r491 into r492;
    not r492 into r493;
    mul 12u8 8u8 into r494;
    shr r0[1u32] r494 into r495;
    and r495 255u128 into r496;
    is.eq r496 0u128 into r497;
    is.eq r496 45u128 into r498;
    or r497 r498 into r499;
    is.eq r496 95u128 into r500;
    or r499 r500 into r501;
    gte r496 48u128 into r502;
    lte r496 57u128 into r503;
    and r502 r503 into r504;
    or r501 r504 into r505;
    gte r496 97u128 into r506;
    lte r496 122u128 into r507;
    and r506 r507 into r508;
    or r505 r508 into r509;
    not r509 into r510;
    mul 13u8 8u8 into r511;
    shr r0[1u32] r511 into r512;
    and r512 255u128 into r513;
    is.eq r513 0u128 into r514;
    is.eq r513 45u128 into r515;
    or r514 r515 into r516;
    is.eq r513 95u128 into r517;
    or r516 r517 into r518;
    gte r513 48u128 into r519;
    lte r513 57u128 into r520;
    and r519 r520 into r521;
    or r518 r521 into r522;
    gte r513 97u128 into r523;
    lte r513 122u128 into r524;
    and r523 r524 into r525;
    or r522 r525 into r526;
    not r526 into r527;
    mul 14u8 8u8 into r528;
    shr r0[1u32] r528 into r529;
    and r529 255u128 into r530;
    is.eq r530 0u128 into r531;
    is.eq r530 45u128 into r532;
    or r531 r532 into r533;
    is.eq r530 95u128 into r534;
    or r533 r534 into r535;
    gte r530 48u128 into r536;
    lte r530 57u128 into r537;
    and r536 r537 into r538;
    or r535 r538 into r539;
    gte r530 97u128 into r540;
    lte r530 122u128 into r541;
    and r540 r541 into r542;
    or r539 r542 into r543;
    not r543 into r544;
    mul 15u8 8u8 into r545;
    shr r0[1u32] r545 into r546;
    and r546 255u128 into r547;
    is.eq r547 0u128 into r548;
    is.eq r547 45u128 into r549;
    or r548 r549 into r550;
    is.eq r547 95u128 into r551;
    or r550 r551 into r552;
    gte r547 48u128 into r553;
    lte r547 57u128 into r554;
    and r553 r554 into r555;
    or r552 r555 into r556;
    gte r547 97u128 into r557;
    lte r547 122u128 into r558;
    and r557 r558 into r559;
    or r556 r559 into r560;
    not r560 into r561;
    ternary r561 false true into r562;
    ternary r544 false r562 into r563;
    ternary r527 false r563 into r564;
    ternary r510 false r564 into r565;
    ternary r493 false r565 into r566;
    ternary r476 false r566 into r567;
    ternary r459 false r567 into r568;
    ternary r442 false r568 into r569;
    ternary r425 false r569 into r570;
    ternary r408 false r570 into r571;
    ternary r391 false r571 into r572;
    ternary r374 false r572 into r573;
    ternary r357 false r573 into r574;
    ternary r340 false r574 into r575;
    ternary r323 false r575 into r576;
    ternary r306 false r576 into r577;
    and r289 r577 into r578;
    mul 0u8 8u8 into r579;
    shr r0[2u32] r579 into r580;
    and r580 255u128 into r581;
    is.eq r581 0u128 into r582;
    is.eq r581 45u128 into r583;
    or r582 r583 into r584;
    is.eq r581 95u128 into r585;
    or r584 r585 into r586;
    gte r581 48u128 into r587;
    lte r581 57u128 into r588;
    and r587 r588 into r589;
    or r586 r589 into r590;
    gte r581 97u128 into r591;
    lte r581 122u128 into r592;
    and r591 r592 into r593;
    or r590 r593 into r594;
    not r594 into r595;
    mul 1u8 8u8 into r596;
    shr r0[2u32] r596 into r597;
    and r597 255u128 into r598;
    is.eq r598 0u128 into r599;
    is.eq r598 45u128 into r600;
    or r599 r600 into r601;
    is.eq r598 95u128 into r602;
    or r601 r602 into r603;
    gte r598 48u128 into r604;
    lte r598 57u128 into r605;
    and r604 r605 into r606;
    or r603 r606 into r607;
    gte r598 97u128 into r608;
    lte r598 122u128 into r609;
    and r608 r609 into r610;
    or r607 r610 into r611;
    not r611 into r612;
    mul 2u8 8u8 into r613;
    shr r0[2u32] r613 into r614;
    and r614 255u128 into r615;
    is.eq r615 0u128 into r616;
    is.eq r615 45u128 into r617;
    or r616 r617 into r618;
    is.eq r615 95u128 into r619;
    or r618 r619 into r620;
    gte r615 48u128 into r621;
    lte r615 57u128 into r622;
    and r621 r622 into r623;
    or r620 r623 into r624;
    gte r615 97u128 into r625;
    lte r615 122u128 into r626;
    and r625 r626 into r627;
    or r624 r627 into r628;
    not r628 into r629;
    mul 3u8 8u8 into r630;
    shr r0[2u32] r630 into r631;
    and r631 255u128 into r632;
    is.eq r632 0u128 into r633;
    is.eq r632 45u128 into r634;
    or r633 r634 into r635;
    is.eq r632 95u128 into r636;
    or r635 r636 into r637;
    gte r632 48u128 into r638;
    lte r632 57u128 into r639;
    and r638 r639 into r640;
    or r637 r640 into r641;
    gte r632 97u128 into r642;
    lte r632 122u128 into r643;
    and r642 r643 into r644;
    or r641 r644 into r645;
    not r645 into r646;
    mul 4u8 8u8 into r647;
    shr r0[2u32] r647 into r648;
    and r648 255u128 into r649;
    is.eq r649 0u128 into r650;
    is.eq r649 45u128 into r651;
    or r650 r651 into r652;
    is.eq r649 95u128 into r653;
    or r652 r653 into r654;
    gte r649 48u128 into r655;
    lte r649 57u128 into r656;
    and r655 r656 into r657;
    or r654 r657 into r658;
    gte r649 97u128 into r659;
    lte r649 122u128 into r660;
    and r659 r660 into r661;
    or r658 r661 into r662;
    not r662 into r663;
    mul 5u8 8u8 into r664;
    shr r0[2u32] r664 into r665;
    and r665 255u128 into r666;
    is.eq r666 0u128 into r667;
    is.eq r666 45u128 into r668;
    or r667 r668 into r669;
    is.eq r666 95u128 into r670;
    or r669 r670 into r671;
    gte r666 48u128 into r672;
    lte r666 57u128 into r673;
    and r672 r673 into r674;
    or r671 r674 into r675;
    gte r666 97u128 into r676;
    lte r666 122u128 into r677;
    and r676 r677 into r678;
    or r675 r678 into r679;
    not r679 into r680;
    mul 6u8 8u8 into r681;
    shr r0[2u32] r681 into r682;
    and r682 255u128 into r683;
    is.eq r683 0u128 into r684;
    is.eq r683 45u128 into r685;
    or r684 r685 into r686;
    is.eq r683 95u128 into r687;
    or r686 r687 into r688;
    gte r683 48u128 into r689;
    lte r683 57u128 into r690;
    and r689 r690 into r691;
    or r688 r691 into r692;
    gte r683 97u128 into r693;
    lte r683 122u128 into r694;
    and r693 r694 into r695;
    or r692 r695 into r696;
    not r696 into r697;
    mul 7u8 8u8 into r698;
    shr r0[2u32] r698 into r699;
    and r699 255u128 into r700;
    is.eq r700 0u128 into r701;
    is.eq r700 45u128 into r702;
    or r701 r702 into r703;
    is.eq r700 95u128 into r704;
    or r703 r704 into r705;
    gte r700 48u128 into r706;
    lte r700 57u128 into r707;
    and r706 r707 into r708;
    or r705 r708 into r709;
    gte r700 97u128 into r710;
    lte r700 122u128 into r711;
    and r710 r711 into r712;
    or r709 r712 into r713;
    not r713 into r714;
    mul 8u8 8u8 into r715;
    shr r0[2u32] r715 into r716;
    and r716 255u128 into r717;
    is.eq r717 0u128 into r718;
    is.eq r717 45u128 into r719;
    or r718 r719 into r720;
    is.eq r717 95u128 into r721;
    or r720 r721 into r722;
    gte r717 48u128 into r723;
    lte r717 57u128 into r724;
    and r723 r724 into r725;
    or r722 r725 into r726;
    gte r717 97u128 into r727;
    lte r717 122u128 into r728;
    and r727 r728 into r729;
    or r726 r729 into r730;
    not r730 into r731;
    mul 9u8 8u8 into r732;
    shr r0[2u32] r732 into r733;
    and r733 255u128 into r734;
    is.eq r734 0u128 into r735;
    is.eq r734 45u128 into r736;
    or r735 r736 into r737;
    is.eq r734 95u128 into r738;
    or r737 r738 into r739;
    gte r734 48u128 into r740;
    lte r734 57u128 into r741;
    and r740 r741 into r742;
    or r739 r742 into r743;
    gte r734 97u128 into r744;
    lte r734 122u128 into r745;
    and r744 r745 into r746;
    or r743 r746 into r747;
    not r747 into r748;
    mul 10u8 8u8 into r749;
    shr r0[2u32] r749 into r750;
    and r750 255u128 into r751;
    is.eq r751 0u128 into r752;
    is.eq r751 45u128 into r753;
    or r752 r753 into r754;
    is.eq r751 95u128 into r755;
    or r754 r755 into r756;
    gte r751 48u128 into r757;
    lte r751 57u128 into r758;
    and r757 r758 into r759;
    or r756 r759 into r760;
    gte r751 97u128 into r761;
    lte r751 122u128 into r762;
    and r761 r762 into r763;
    or r760 r763 into r764;
    not r764 into r765;
    mul 11u8 8u8 into r766;
    shr r0[2u32] r766 into r767;
    and r767 255u128 into r768;
    is.eq r768 0u128 into r769;
    is.eq r768 45u128 into r770;
    or r769 r770 into r771;
    is.eq r768 95u128 into r772;
    or r771 r772 into r773;
    gte r768 48u128 into r774;
    lte r768 57u128 into r775;
    and r774 r775 into r776;
    or r773 r776 into r777;
    gte r768 97u128 into r778;
    lte r768 122u128 into r779;
    and r778 r779 into r780;
    or r777 r780 into r781;
    not r781 into r782;
    mul 12u8 8u8 into r783;
    shr r0[2u32] r783 into r784;
    and r784 255u128 into r785;
    is.eq r785 0u128 into r786;
    is.eq r785 45u128 into r787;
    or r786 r787 into r788;
    is.eq r785 95u128 into r789;
    or r788 r789 into r790;
    gte r785 48u128 into r791;
    lte r785 57u128 into r792;
    and r791 r792 into r793;
    or r790 r793 into r794;
    gte r785 97u128 into r795;
    lte r785 122u128 into r796;
    and r795 r796 into r797;
    or r794 r797 into r798;
    not r798 into r799;
    mul 13u8 8u8 into r800;
    shr r0[2u32] r800 into r801;
    and r801 255u128 into r802;
    is.eq r802 0u128 into r803;
    is.eq r802 45u128 into r804;
    or r803 r804 into r805;
    is.eq r802 95u128 into r806;
    or r805 r806 into r807;
    gte r802 48u128 into r808;
    lte r802 57u128 into r809;
    and r808 r809 into r810;
    or r807 r810 into r811;
    gte r802 97u128 into r812;
    lte r802 122u128 into r813;
    and r812 r813 into r814;
    or r811 r814 into r815;
    not r815 into r816;
    mul 14u8 8u8 into r817;
    shr r0[2u32] r817 into r818;
    and r818 255u128 into r819;
    is.eq r819 0u128 into r820;
    is.eq r819 45u128 into r821;
    or r820 r821 into r822;
    is.eq r819 95u128 into r823;
    or r822 r823 into r824;
    gte r819 48u128 into r825;
    lte r819 57u128 into r826;
    and r825 r826 into r827;
    or r824 r827 into r828;
    gte r819 97u128 into r829;
    lte r819 122u128 into r830;
    and r829 r830 into r831;
    or r828 r831 into r832;
    not r832 into r833;
    mul 15u8 8u8 into r834;
    shr r0[2u32] r834 into r835;
    and r835 255u128 into r836;
    is.eq r836 0u128 into r837;
    is.eq r836 45u128 into r838;
    or r837 r838 into r839;
    is.eq r836 95u128 into r840;
    or r839 r840 into r841;
    gte r836 48u128 into r842;
    lte r836 57u128 into r843;
    and r842 r843 into r844;
    or r841 r844 into r845;
    gte r836 97u128 into r846;
    lte r836 122u128 into r847;
    and r846 r847 into r848;
    or r845 r848 into r849;
    not r849 into r850;
    ternary r850 false true into r851;
    ternary r833 false r851 into r852;
    ternary r816 false r852 into r853;
    ternary r799 false r853 into r854;
    ternary r782 false r854 into r855;
    ternary r765 false r855 into r856;
    ternary r748 false r856 into r857;
    ternary r731 false r857 into r858;
    ternary r714 false r858 into r859;
    ternary r697 false r859 into r860;
    ternary r680 false r860 into r861;
    ternary r663 false r861 into r862;
    ternary r646 false r862 into r863;
    ternary r629 false r863 into r864;
    ternary r612 false r864 into r865;
    ternary r595 false r865 into r866;
    and r578 r866 into r867;
    mul 0u8 8u8 into r868;
    shr r0[3u32] r868 into r869;
    and r869 255u128 into r870;
    is.eq r870 0u128 into r871;
    is.eq r870 45u128 into r872;
    or r871 r872 into r873;
    is.eq r870 95u128 into r874;
    or r873 r874 into r875;
    gte r870 48u128 into r876;
    lte r870 57u128 into r877;
    and r876 r877 into r878;
    or r875 r878 into r879;
    gte r870 97u128 into r880;
    lte r870 122u128 into r881;
    and r880 r881 into r882;
    or r879 r882 into r883;
    not r883 into r884;
    mul 1u8 8u8 into r885;
    shr r0[3u32] r885 into r886;
    and r886 255u128 into r887;
    is.eq r887 0u128 into r888;
    is.eq r887 45u128 into r889;
    or r888 r889 into r890;
    is.eq r887 95u128 into r891;
    or r890 r891 into r892;
    gte r887 48u128 into r893;
    lte r887 57u128 into r894;
    and r893 r894 into r895;
    or r892 r895 into r896;
    gte r887 97u128 into r897;
    lte r887 122u128 into r898;
    and r897 r898 into r899;
    or r896 r899 into r900;
    not r900 into r901;
    mul 2u8 8u8 into r902;
    shr r0[3u32] r902 into r903;
    and r903 255u128 into r904;
    is.eq r904 0u128 into r905;
    is.eq r904 45u128 into r906;
    or r905 r906 into r907;
    is.eq r904 95u128 into r908;
    or r907 r908 into r909;
    gte r904 48u128 into r910;
    lte r904 57u128 into r911;
    and r910 r911 into r912;
    or r909 r912 into r913;
    gte r904 97u128 into r914;
    lte r904 122u128 into r915;
    and r914 r915 into r916;
    or r913 r916 into r917;
    not r917 into r918;
    mul 3u8 8u8 into r919;
    shr r0[3u32] r919 into r920;
    and r920 255u128 into r921;
    is.eq r921 0u128 into r922;
    is.eq r921 45u128 into r923;
    or r922 r923 into r924;
    is.eq r921 95u128 into r925;
    or r924 r925 into r926;
    gte r921 48u128 into r927;
    lte r921 57u128 into r928;
    and r927 r928 into r929;
    or r926 r929 into r930;
    gte r921 97u128 into r931;
    lte r921 122u128 into r932;
    and r931 r932 into r933;
    or r930 r933 into r934;
    not r934 into r935;
    mul 4u8 8u8 into r936;
    shr r0[3u32] r936 into r937;
    and r937 255u128 into r938;
    is.eq r938 0u128 into r939;
    is.eq r938 45u128 into r940;
    or r939 r940 into r941;
    is.eq r938 95u128 into r942;
    or r941 r942 into r943;
    gte r938 48u128 into r944;
    lte r938 57u128 into r945;
    and r944 r945 into r946;
    or r943 r946 into r947;
    gte r938 97u128 into r948;
    lte r938 122u128 into r949;
    and r948 r949 into r950;
    or r947 r950 into r951;
    not r951 into r952;
    mul 5u8 8u8 into r953;
    shr r0[3u32] r953 into r954;
    and r954 255u128 into r955;
    is.eq r955 0u128 into r956;
    is.eq r955 45u128 into r957;
    or r956 r957 into r958;
    is.eq r955 95u128 into r959;
    or r958 r959 into r960;
    gte r955 48u128 into r961;
    lte r955 57u128 into r962;
    and r961 r962 into r963;
    or r960 r963 into r964;
    gte r955 97u128 into r965;
    lte r955 122u128 into r966;
    and r965 r966 into r967;
    or r964 r967 into r968;
    not r968 into r969;
    mul 6u8 8u8 into r970;
    shr r0[3u32] r970 into r971;
    and r971 255u128 into r972;
    is.eq r972 0u128 into r973;
    is.eq r972 45u128 into r974;
    or r973 r974 into r975;
    is.eq r972 95u128 into r976;
    or r975 r976 into r977;
    gte r972 48u128 into r978;
    lte r972 57u128 into r979;
    and r978 r979 into r980;
    or r977 r980 into r981;
    gte r972 97u128 into r982;
    lte r972 122u128 into r983;
    and r982 r983 into r984;
    or r981 r984 into r985;
    not r985 into r986;
    mul 7u8 8u8 into r987;
    shr r0[3u32] r987 into r988;
    and r988 255u128 into r989;
    is.eq r989 0u128 into r990;
    is.eq r989 45u128 into r991;
    or r990 r991 into r992;
    is.eq r989 95u128 into r993;
    or r992 r993 into r994;
    gte r989 48u128 into r995;
    lte r989 57u128 into r996;
    and r995 r996 into r997;
    or r994 r997 into r998;
    gte r989 97u128 into r999;
    lte r989 122u128 into r1000;
    and r999 r1000 into r1001;
    or r998 r1001 into r1002;
    not r1002 into r1003;
    mul 8u8 8u8 into r1004;
    shr r0[3u32] r1004 into r1005;
    and r1005 255u128 into r1006;
    is.eq r1006 0u128 into r1007;
    is.eq r1006 45u128 into r1008;
    or r1007 r1008 into r1009;
    is.eq r1006 95u128 into r1010;
    or r1009 r1010 into r1011;
    gte r1006 48u128 into r1012;
    lte r1006 57u128 into r1013;
    and r1012 r1013 into r1014;
    or r1011 r1014 into r1015;
    gte r1006 97u128 into r1016;
    lte r1006 122u128 into r1017;
    and r1016 r1017 into r1018;
    or r1015 r1018 into r1019;
    not r1019 into r1020;
    mul 9u8 8u8 into r1021;
    shr r0[3u32] r1021 into r1022;
    and r1022 255u128 into r1023;
    is.eq r1023 0u128 into r1024;
    is.eq r1023 45u128 into r1025;
    or r1024 r1025 into r1026;
    is.eq r1023 95u128 into r1027;
    or r1026 r1027 into r1028;
    gte r1023 48u128 into r1029;
    lte r1023 57u128 into r1030;
    and r1029 r1030 into r1031;
    or r1028 r1031 into r1032;
    gte r1023 97u128 into r1033;
    lte r1023 122u128 into r1034;
    and r1033 r1034 into r1035;
    or r1032 r1035 into r1036;
    not r1036 into r1037;
    mul 10u8 8u8 into r1038;
    shr r0[3u32] r1038 into r1039;
    and r1039 255u128 into r1040;
    is.eq r1040 0u128 into r1041;
    is.eq r1040 45u128 into r1042;
    or r1041 r1042 into r1043;
    is.eq r1040 95u128 into r1044;
    or r1043 r1044 into r1045;
    gte r1040 48u128 into r1046;
    lte r1040 57u128 into r1047;
    and r1046 r1047 into r1048;
    or r1045 r1048 into r1049;
    gte r1040 97u128 into r1050;
    lte r1040 122u128 into r1051;
    and r1050 r1051 into r1052;
    or r1049 r1052 into r1053;
    not r1053 into r1054;
    mul 11u8 8u8 into r1055;
    shr r0[3u32] r1055 into r1056;
    and r1056 255u128 into r1057;
    is.eq r1057 0u128 into r1058;
    is.eq r1057 45u128 into r1059;
    or r1058 r1059 into r1060;
    is.eq r1057 95u128 into r1061;
    or r1060 r1061 into r1062;
    gte r1057 48u128 into r1063;
    lte r1057 57u128 into r1064;
    and r1063 r1064 into r1065;
    or r1062 r1065 into r1066;
    gte r1057 97u128 into r1067;
    lte r1057 122u128 into r1068;
    and r1067 r1068 into r1069;
    or r1066 r1069 into r1070;
    not r1070 into r1071;
    mul 12u8 8u8 into r1072;
    shr r0[3u32] r1072 into r1073;
    and r1073 255u128 into r1074;
    is.eq r1074 0u128 into r1075;
    is.eq r1074 45u128 into r1076;
    or r1075 r1076 into r1077;
    is.eq r1074 95u128 into r1078;
    or r1077 r1078 into r1079;
    gte r1074 48u128 into r1080;
    lte r1074 57u128 into r1081;
    and r1080 r1081 into r1082;
    or r1079 r1082 into r1083;
    gte r1074 97u128 into r1084;
    lte r1074 122u128 into r1085;
    and r1084 r1085 into r1086;
    or r1083 r1086 into r1087;
    not r1087 into r1088;
    mul 13u8 8u8 into r1089;
    shr r0[3u32] r1089 into r1090;
    and r1090 255u128 into r1091;
    is.eq r1091 0u128 into r1092;
    is.eq r1091 45u128 into r1093;
    or r1092 r1093 into r1094;
    is.eq r1091 95u128 into r1095;
    or r1094 r1095 into r1096;
    gte r1091 48u128 into r1097;
    lte r1091 57u128 into r1098;
    and r1097 r1098 into r1099;
    or r1096 r1099 into r1100;
    gte r1091 97u128 into r1101;
    lte r1091 122u128 into r1102;
    and r1101 r1102 into r1103;
    or r1100 r1103 into r1104;
    not r1104 into r1105;
    mul 14u8 8u8 into r1106;
    shr r0[3u32] r1106 into r1107;
    and r1107 255u128 into r1108;
    is.eq r1108 0u128 into r1109;
    is.eq r1108 45u128 into r1110;
    or r1109 r1110 into r1111;
    is.eq r1108 95u128 into r1112;
    or r1111 r1112 into r1113;
    gte r1108 48u128 into r1114;
    lte r1108 57u128 into r1115;
    and r1114 r1115 into r1116;
    or r1113 r1116 into r1117;
    gte r1108 97u128 into r1118;
    lte r1108 122u128 into r1119;
    and r1118 r1119 into r1120;
    or r1117 r1120 into r1121;
    not r1121 into r1122;
    mul 15u8 8u8 into r1123;
    shr r0[3u32] r1123 into r1124;
    and r1124 255u128 into r1125;
    is.eq r1125 0u128 into r1126;
    is.eq r1125 45u128 into r1127;
    or r1126 r1127 into r1128;
    is.eq r1125 95u128 into r1129;
    or r1128 r1129 into r1130;
    gte r1125 48u128 into r1131;
    lte r1125 57u128 into r1132;
    and r1131 r1132 into r1133;
    or r1130 r1133 into r1134;
    gte r1125 97u128 into r1135;
    lte r1125 122u128 into r1136;
    and r1135 r1136 into r1137;
    or r1134 r1137 into r1138;
    not r1138 into r1139;
    ternary r1139 false true into r1140;
    ternary r1122 false r1140 into r1141;
    ternary r1105 false r1141 into r1142;
    ternary r1088 false r1142 into r1143;
    ternary r1071 false r1143 into r1144;
    ternary r1054 false r1144 into r1145;
    ternary r1037 false r1145 into r1146;
    ternary r1020 false r1146 into r1147;
    ternary r1003 false r1147 into r1148;
    ternary r986 false r1148 into r1149;
    ternary r969 false r1149 into r1150;
    ternary r952 false r1150 into r1151;
    ternary r935 false r1151 into r1152;
    ternary r918 false r1152 into r1153;
    ternary r901 false r1153 into r1154;
    ternary r884 false r1154 into r1155;
    and r867 r1155 into r1156;
    output r1156 as boolean;

function initialize_collection:
    input r0 as u128.public;
    input r1 as u128.public;
    input r2 as BaseURI.public;
    hash.bhp256 self.caller into r3 as u128;
    async initialize_collection r0 r1 r3 r2 into r4;
    output r4 as aleo_name_service_registry_v3.aleo/initialize_collection.future;

finalize initialize_collection:
    input r0 as u128.public;
    input r1 as u128.public;
    input r2 as u128.public;
    input r3 as BaseURI.public;
    get.or_use toggle_settings[0u8] 0u32 into r4;
    and r4 1u32 into r5;
    assert.eq r5 0u32 ;
    set r0 into general_settings[0u8];
    set r0 into general_settings[1u8];
    set r1 into general_settings[2u8];
    set r3.data0 into general_settings[3u8];
    set r3.data1 into general_settings[4u8];
    set r3.data2 into general_settings[5u8];
    set r3.data3 into general_settings[6u8];
    set r2 into general_settings[7u8];
    set 1u32 into toggle_settings[0u8];
    set 0u32 into toggle_settings[1u8];

function set_admin:
    input r0 as address.private;
    hash.bhp256 self.caller into r1 as u128;
    hash.bhp256 r0 into r2 as u128;
    async set_admin r1 r2 into r3;
    output r3 as aleo_name_service_registry_v3.aleo/set_admin.future;

finalize set_admin:
    input r0 as u128.public;
    input r1 as u128.public;
    get general_settings[7u8] into r2;
    assert.eq r0 r2 ;
    set r1 into general_settings[7u8];

function set_mint_block:
    input r0 as u32.public;
    hash.bhp256 self.caller into r1 as u128;
    async set_mint_block r1 r0 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/set_mint_block.future;

finalize set_mint_block:
    input r0 as u128.public;
    input r1 as u32.public;
    get general_settings[7u8] into r2;
    assert.eq r0 r2 ;
    get toggle_settings[0u8] into r3;
    and r3 9u32 into r4;
    assert.eq r4 1u32 ;
    set r1 into toggle_settings[1u8];

function update_base_uri:
    input r0 as BaseURI.public;
    hash.bhp256 self.caller into r1 as u128;
    async update_base_uri r1 r0 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/update_base_uri.future;

finalize update_base_uri:
    input r0 as u128.public;
    input r1 as BaseURI.public;
    get general_settings[7u8] into r2;
    assert.eq r0 r2 ;
    get toggle_settings[0u8] into r3;
    and r3 9u32 into r4;
    assert.eq r4 1u32 ;
    set r1.data0 into general_settings[3u8];
    set r1.data1 into general_settings[4u8];
    set r1.data2 into general_settings[5u8];
    set r1.data3 into general_settings[6u8];

function update_toggle_settings:
    input r0 as u32.public;
    hash.bhp256 self.caller into r1 as u128;
    async update_toggle_settings r1 r0 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/update_toggle_settings.future;

finalize update_toggle_settings:
    input r0 as u128.public;
    input r1 as u32.public;
    get general_settings[7u8] into r2;
    assert.eq r0 r2 ;
    get toggle_settings[0u8] into r3;
    and r3 9u32 into r4;
    assert.eq r4 1u32 ;
    and r1 1u32 into r5;
    assert.eq r5 1u32 ;
    set r1 into toggle_settings[0u8];

function register_tld:
    input r0 as address.private;
    input r1 as [u128; 4u32].private;
    cast r1 0field into r2 as Name;
    hash.psd2 r2 into r3 as field;
    hash.bhp256 self.caller into r4 as u128;
    async register_tld r4 r0 r3 r1 into r5;
    output r5 as aleo_name_service_registry_v3.aleo/register_tld.future;

finalize register_tld:
    input r0 as u128.public;
    input r1 as address.public;
    input r2 as field.public;
    input r3 as [u128; 4u32].public;
    get general_settings[7u8] into r4;
    assert.eq r0 r4 ;
    cast r3 0field 0u128 into r5 as NameStruct;
    set r5 into names[r2];
    set r1 into nft_owners[r2];

function register:
    input r0 as [u128; 4u32].private;
    input r1 as field.private;
    input r2 as address.private;
    input r3 as u128.private;
    call validate_name r0 into r4;
    assert.eq r4 true ;
    cast r0 r1 into r5 as Name;
    hash.psd2 r5 into r6 as field;
    cast r2 r6 0scalar into r7 as NFT.record;
    async register r6 r0 r1 r3 self.caller into r8;
    output r7 as NFT.record;
    output r8 as aleo_name_service_registry_v3.aleo/register.future;

finalize register:
    input r0 as field.public;
    input r1 as [u128; 4u32].public;
    input r2 as field.public;
    input r3 as u128.public;
    input r4 as address.public;
    get nft_owners[r2] into r5;
    assert.eq r4 r5 ;
    contains names[r0] into r6;
    assert.eq r6 false ;
    get toggle_settings[1u8] into r7;
    lte r7 block.height into r8;
    assert.eq r8 true ;
    get toggle_settings[0u8] into r9;
    and r9 3u32 into r10;
    assert.eq r10 3u32 ;
    cast r1 r2 r3 into r11 as NameStruct;
    set r11 into names[r0];

function register_private:
    input r0 as [u128; 4u32].private;
    input r1 as NFT.record;
    input r2 as address.private;
    input r3 as u128.private;
    call validate_name r0 into r4;
    assert.eq r4 true ;
    cast r0 r1.data into r5 as Name;
    hash.psd2 r5 into r6 as field;
    cast r2 r6 0scalar into r7 as NFT.record;
    cast r1.owner r1.data r1.edition into r8 as NFT.record;
    async register_private r6 r0 r1.data r3 into r9;
    output r7 as NFT.record;
    output r8 as NFT.record;
    output r9 as aleo_name_service_registry_v3.aleo/register_private.future;

finalize register_private:
    input r0 as field.public;
    input r1 as [u128; 4u32].public;
    input r2 as field.public;
    input r3 as u128.public;
    contains names[r0] into r4;
    assert.eq r4 false ;
    get toggle_settings[1u8] into r5;
    lte r5 block.height into r6;
    assert.eq r6 true ;
    get toggle_settings[0u8] into r7;
    and r7 3u32 into r8;
    assert.eq r8 3u32 ;
    cast r1 r2 r3 into r9 as NameStruct;
    set r9 into names[r0];

function register_public:
    input r0 as [u128; 4u32].private;
    input r1 as field.private;
    input r2 as address.private;
    input r3 as u128.private;
    call validate_name r0 into r4;
    assert.eq r4 true ;
    cast r0 r1 into r5 as Name;
    hash.psd2 r5 into r6 as field;
    cast r2 r6 0scalar into r7 as NFT.record;
    async register_public r6 r0 r1 r3 self.signer into r8;
    output r7 as NFT.record;
    output r8 as aleo_name_service_registry_v3.aleo/register_public.future;

finalize register_public:
    input r0 as field.public;
    input r1 as [u128; 4u32].public;
    input r2 as field.public;
    input r3 as u128.public;
    input r4 as address.public;
    get nft_owners[r2] into r5;
    assert.eq r4 r5 ;
    contains names[r0] into r6;
    assert.eq r6 false ;
    get toggle_settings[1u8] into r7;
    lte r7 block.height into r8;
    assert.eq r8 true ;
    get toggle_settings[0u8] into r9;
    and r9 3u32 into r10;
    assert.eq r10 3u32 ;
    cast r1 r2 r3 into r11 as NameStruct;
    set r11 into names[r0];

function transfer_private:
    input r0 as NFT.record;
    input r1 as address.private;
    cast r1 r0.data 0scalar into r2 as NFT.record;
    output r2 as NFT.record;

function transfer_public:
    input r0 as address.private;
    input r1 as field.private;
    async transfer_public r0 r1 self.caller into r2;
    output r2 as aleo_name_service_registry_v3.aleo/transfer_public.future;

finalize transfer_public:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as address.public;
    assert.neq r2 r0 ;
    get nft_owners[r1] into r3;
    cast r1 r3 into r4 as NameAddressPair;
    cast r3 r2 into r5 as AddressPair;
    is.eq r2 r3 into r6;
    get.or_use approved_for_nft[r4] r3 into r7;
    is.eq r7 r2 into r8;
    or r6 r8 into r9;
    get.or_use approved_for_all[r5] false into r10;
    or r9 r10 into r11;
    assert.eq r11 true ;
    set r0 into nft_owners[r1];
    get.or_use name_versions[r1] 1u64 into r12;
    add r12 1u64 into r13;
    set r13 into name_versions[r1];
    get.or_use primary_names[r3] 0field into r14;
    is.eq r14 r1 into r15;
    branch.eq r15 false to end;
    remove primary_names[r3];
    position end;

function convert_private_to_public:
    input r0 as NFT.record;
    input r1 as address.private;
    async convert_private_to_public r1 r0.data into r2;
    output r2 as aleo_name_service_registry_v3.aleo/convert_private_to_public.future;

finalize convert_private_to_public:
    input r0 as address.public;
    input r1 as field.public;
    set r0 into nft_owners[r1];

function convert_public_to_private:
    input r0 as field.private;
    input r1 as address.private;
    cast r1 r0 0scalar into r2 as NFT.record;
    async convert_public_to_private self.caller r0 into r3;
    output r2 as NFT.record;
    output r3 as aleo_name_service_registry_v3.aleo/convert_public_to_private.future;

finalize convert_public_to_private:
    input r0 as address.public;
    input r1 as field.public;
    get nft_owners[r1] into r2;
    cast r1 r2 into r3 as NameAddressPair;
    cast r2 r0 into r4 as AddressPair;
    is.eq r0 r2 into r5;
    get.or_use approved_for_nft[r3] r2 into r6;
    is.eq r6 r0 into r7;
    or r5 r7 into r8;
    get.or_use approved_for_all[r4] false into r9;
    or r8 r9 into r10;
    assert.eq r10 true ;
    remove nft_owners[r1];
    get.or_use name_versions[r1] 1u64 into r11;
    add r11 1u64 into r12;
    set r12 into name_versions[r1];
    get.or_use primary_names[r2] 0field into r13;
    is.eq r13 r1 into r14;
    branch.eq r14 false to end;
    remove primary_names[r2];
    position end;

function set_primary_name:
    input r0 as field.private;
    async set_primary_name r0 self.caller into r1;
    output r1 as aleo_name_service_registry_v3.aleo/set_primary_name.future;

finalize set_primary_name:
    input r0 as field.public;
    input r1 as address.public;
    get nft_owners[r0] into r2;
    assert.eq r1 r2 ;
    set r0 into primary_names[r1];

function unset_primary_name:
    async unset_primary_name self.caller into r0;
    output r0 as aleo_name_service_registry_v3.aleo/unset_primary_name.future;

finalize unset_primary_name:
    input r0 as address.public;
    remove primary_names[r0];

function set_resolver:
    input r0 as field.private;
    input r1 as u128.private;
    async set_resolver r0 self.caller r1 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/set_resolver.future;

finalize set_resolver:
    input r0 as field.public;
    input r1 as address.public;
    input r2 as u128.public;
    get nft_owners[r0] into r3;
    assert.eq r1 r3 ;
    get names[r0] into r4;
    cast r4.name r4.parent r2 into r5 as NameStruct;
    set r5 into names[r0];

function set_resolver_record:
    input r0 as field.private;
    input r1 as u128.private;
    input r2 as [u128; 8u32].private;
    gt r1 0u128 into r3;
    assert.eq r3 true ;
    gt r2[0u32] 0u128 into r4;
    assert.eq r4 true ;
    async set_resolver_record r0 self.caller r1 r2 into r5;
    output r5 as aleo_name_service_registry_v3.aleo/set_resolver_record.future;

finalize set_resolver_record:
    input r0 as field.public;
    input r1 as address.public;
    input r2 as u128.public;
    input r3 as [u128; 8u32].public;
    get nft_owners[r0] into r4;
    assert.eq r1 r4 ;
    get.or_use name_versions[r0] 1u64 into r5;
    cast r0 r2 r5 into r6 as ResolverIndex;
    set r3 into resolvers[r6];

function unset_resolver_record:
    input r0 as field.private;
    input r1 as u128.private;
    async unset_resolver_record r0 self.caller r1 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/unset_resolver_record.future;

finalize unset_resolver_record:
    input r0 as field.public;
    input r1 as address.public;
    input r2 as u128.public;
    get nft_owners[r0] into r3;
    assert.eq r1 r3 ;
    get.or_use name_versions[r0] 1u64 into r4;
    cast r0 r2 r4 into r5 as ResolverIndex;
    remove resolvers[r5];

function clear_resolver_record:
    input r0 as field.private;
    async clear_resolver_record r0 self.caller into r1;
    output r1 as aleo_name_service_registry_v3.aleo/clear_resolver_record.future;

finalize clear_resolver_record:
    input r0 as field.public;
    input r1 as address.public;
    get nft_owners[r0] into r2;
    assert.eq r1 r2 ;
    get.or_use name_versions[r0] 1u64 into r3;
    add r3 1u64 into r4;
    set r4 into name_versions[r0];

function is_owner:
    input r0 as field.private;
    input r1 as address.private;
    async is_owner r0 r1 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/is_owner.future;

finalize is_owner:
    input r0 as field.public;
    input r1 as address.public;
    get nft_owners[r0] into r2;
    assert.eq r1 r2 ;

function proof_ownership:
    input r0 as NFT.record;
    input r1 as address.private;
    cast r1 r0.owner r0.data r0.edition into r2 as NFT_ownership.record;
    cast r0.owner r0.data r0.edition into r3 as NFT.record;
    output r3 as NFT.record;
    output r2 as NFT_ownership.record;

function authorize:
    input r0 as NFT.record;
    async authorize into r1;
    output r1 as aleo_name_service_registry_v3.aleo/authorize.future;

finalize authorize:
    assert.eq 0u8 1u8 ;

function burn:
    input r0 as NFT.record;
    async burn r0.data into r1;
    output r1 as aleo_name_service_registry_v3.aleo/burn.future;

finalize burn:
    input r0 as field.public;
    remove names[r0];

function is_available:
    input r0 as field.private;
    async is_available r0 into r1;
    output r1 as aleo_name_service_registry_v3.aleo/is_available.future;

finalize is_available:
    input r0 as field.public;
    contains names[r0] into r1;
    assert.eq r1 false ;

function approve:
    input r0 as address.private;
    input r1 as field.private;
    async approve self.caller r0 r1 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/approve.future;

finalize approve:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as field.public;
    get nft_owners[r2] into r3;
    assert.eq r0 r3 ;
    cast r2 r0 into r4 as NameAddressPair;
    set r1 into approved_for_nft[r4];

function set_approval_for_all:
    input r0 as address.private;
    input r1 as boolean.private;
    async set_approval_for_all self.caller r0 r1 into r2;
    output r2 as aleo_name_service_registry_v3.aleo/set_approval_for_all.future;

finalize set_approval_for_all:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as boolean.public;
    cast r0 r1 into r3 as AddressPair;
    set r2 into approved_for_all[r3];

        "#;

        let mut imports: HashMap<String, String> = HashMap::new();

        imports.insert(String::from(import_name_2), String::from(import_code_2));
        imports.insert(String::from(import_name_1), String::from(import_code_1));

        let base_fee = commands::deployment_cost(program_code, Some(imports));
        println!("cost_deployment_fee: {}", base_fee.unwrap());
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
                    "signer": "aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf",
                    "network": "0u16",
                    "program": "credits.aleo",
                    "function": "transfer_public",
                    "input_ids": [
                        {
                            "type": "public",
                            "id": "6960367568841100099474293627027684811096195558479708440748130274059888724370field"
                        },
                        {
                            "type": "public",
                            "id": "6086094073051805908770717807830388239700425842953925348815574119573997976269field"
                        }
                    ],
                    "inputs": [
                        "aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf",
                        "666000000u64"
                    ],
                    "signature": "sign1p3kjn46qkg3fex30ncgc8lgfq6kpmsjc32rwxtwd3yn4t67u3uql3zl9hdt3ltmrgdgvdsuygp68qfle5kuaagfgu5j6d7qt78epqqe9ns2t5wheecqdz6lzdparem07pkf4rvz8xjlgk2qgtfjucw5cpvn668xhpvmuzh57rsr0ruvdkfv4wcsqvx38sfjvf5w6tmgfx0apztznfsv",
                    "sk_tag": "1855276397120242374951049861458305672602156658031151067189099605476050215845field",
                    "tvk": "3945417340138746886969664449991498349216705800808138846940355100960477906385field",
                    "tcm": "4676576906372754186034273661793409431395658343304286253459887243997320858275field",
                    "scm": "1990356926948239390837988840476153202734181598257881301411001429887688598295field"
                }
            ],
            "transitions": [
                {
                    "id": "au18zaskppmurqeq8erw0t220jgx7myp7kkaxll24d54sz7adwew5gqp3grmt",
                    "program": "credits.aleo",
                    "function": "transfer_public",
                    "inputs": [
                        {
                            "type": "public",
                            "id": "6960367568841100099474293627027684811096195558479708440748130274059888724370field",
                            "value": "aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf"
                        },
                        {
                            "type": "public",
                            "id": "6086094073051805908770717807830388239700425842953925348815574119573997976269field",
                            "value": "666000000u64"
                        }
                    ],
                    "outputs": [
                        {
                            "type": "future",
                            "id": "4855584176600217008354116186996626902589886047650619036671440671797040319652field",
                            "value": "{\n  program_id: credits.aleo,\n  function_name: transfer_public,\n  arguments: [\n    aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf,\n    aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf,\n    666000000u64\n  ]\n}"
                        }
                    ],
                    "tpk": "3620252757780396101806511571211648885114762588889080603685867090727000385571group",
                    "tcm": "4676576906372754186034273661793409431395658343304286253459887243997320858275field",
                    "scm": "1990356926948239390837988840476153202734181598257881301411001429887688598295field"
                }
            ]
        }
    "#;
        let fee_authorization = r#"
        {
            "requests": [
                {
                    "signer": "aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf",
                    "network": "0u16",
                    "program": "credits.aleo",
                    "function": "fee_public",
                    "input_ids": [
                        {
                            "type": "public",
                            "id": "3526123591031330683629340030700784464070921235461458542665331471471716019781field"
                        },
                        {
                            "type": "public",
                            "id": "6999512932848483882969787875194669080179613625094375374391524614301005725field"
                        },
                        {
                            "type": "public",
                            "id": "5468717336085836504135073500410465879533663485051944173698019281684710133954field"
                        }
                    ],
                    "inputs": [
                        "51060u64",
                        "0u64",
                        "4449089711458026441230058091278413281669491666139294380845359105913988943404field"
                    ],
                    "signature": "sign1tj9khek7kf9zul4v8yw8wdquaufc8n87p84ndfp0s5ug4lq2wcpe3a85cjejcnpxlld8xxkscu3ptd4mnw9hex9dqqdrcvpt3ahzvqe9ns2t5wheecqdz6lzdparem07pkf4rvz8xjlgk2qgtfjucw5cpvn668xhpvmuzh57rsr0ruvdkfv4wcsqvx38sfjvf5w6tmgfx0apzs6zpxl",
                    "sk_tag": "1855276397120242374951049861458305672602156658031151067189099605476050215845field",
                    "tvk": "3768567974815247516521768678654540475004435385897218163598249361654945534274field",
                    "tcm": "8433738589389145343699492130977422942936262977505081301762101570093316027671field",
                    "scm": "4798236681357478635240296248380629876771790491157180068049958052656755197474field"
                }
            ],
            "transitions": [
                {
                    "id": "au1q9mch7her2r3pdy9sp7vf48nv9erj8uezfj2g5hatgp9umx2h5rqq2g8tw",
                    "program": "credits.aleo",
                    "function": "fee_public",
                    "inputs": [
                        {
                            "type": "public",
                            "id": "3526123591031330683629340030700784464070921235461458542665331471471716019781field",
                            "value": "51060u64"
                        },
                        {
                            "type": "public",
                            "id": "6999512932848483882969787875194669080179613625094375374391524614301005725field",
                            "value": "0u64"
                        },
                        {
                            "type": "public",
                            "id": "5468717336085836504135073500410465879533663485051944173698019281684710133954field",
                            "value": "4449089711458026441230058091278413281669491666139294380845359105913988943404field"
                        }
                    ],
                    "outputs": [
                        {
                            "type": "future",
                            "id": "7831022766136850774635272260505930434151030068657637544437449865185755136176field",
                            "value": "{\n  program_id: credits.aleo,\n  function_name: fee_public,\n  arguments: [\n    aleo1mmjtyljs4stmgp4dazwnycfv564gajanvt2kfc37hj7zcstzuyqq9tq2gf,\n    51060u64\n  ]\n}"
                        }
                    ],
                    "tpk": "7911165430492751161476656445128150516954731774997033158436422447630907438806group",
                    "tcm": "8433738589389145343699492130977422942936262977505081301762101570093316027671field",
                    "scm": "4798236681357478635240296248380629876771790491157180068049958052656755197474field"
                }
            ]
        }
    "#;

        let query = Some("https://mainnetv0.sotertech.io");

        let transaction = commands::transaction_for_authorize(
            "credits.aleo",
            execute_authorization,
            fee_authorization,
            query,
        );
        println!("transfer transaction: {}", transaction.unwrap());
    }

    #[test]
    fn deploy_for_authorize() {
        let query = Some("https://mainnetv0.sotertech.io");
        let program = r#"
        program hello_world_3db52e57.aleo;



function main:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
        "#;
        let owner_str = r#"
        {
            "address": "aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns",
            "signature": "sign1m4zt4zc2rnjl3c0hr5jgc9vsft2q8kkgy8454mnqvk5hwzjm3ypcg3wh4dxnlw5fq9ld96ykwka6uj7pnggw36ggr5uapk5z6lq0jqg32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsjk2g7n6"
          }
        "#;

        let fee_authorization_str = r#"
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
                      "id": "7461282104433366449900697799474885740026008139066330886969703322575758055217field"
                    },
                    {
                      "type": "public",
                      "id": "3934803756888630496501816151050540292915193353802102899830239800849008699942field"
                    },
                    {
                      "type": "public",
                      "id": "2704568184889906689601787863190035940222705911013024881738685099334624579190field"
                    }
                  ],
                  "inputs": [
                    "5000000u64",
                    "1000000u64",
                    "5143289311869305992819731554276892145943690667501397210953951405716358737411field"
                  ],
                  "signature": "sign1lat9vedqjhyjgln2ygzclxj9fe25kzmdlzfcmdt3x63krfhr3vptug7fjygl99qzk5yjsn9k30fhpnwjj3rsnldhshwtlauj6q525pq32v75vccfdluwzulwl6p3h9254xvwn0unk8rj97pg26sy763cp7hlx2fvrlm38032pywgm9tdc969sn7nv3md34e9z9qergtruxmsj002rpy",
                  "sk_tag": "8392991161530139852499528373585292010003647568520892791643421634970601392470field",
                  "tvk": "7038345925362083712533035838095457463996994870127572591579114932265713415059field",
                  "tcm": "28343811668558083092323437539573502422062762506109415690591179678071251191field"
                }
              ],
              "transitions": [
                {
                  "id": "au1g0qxr5z4lgltpnmalltcnx3ddy62rn2vmc3x53lymulsyydeyy8scvun5t",
                  "program": "credits.aleo",
                  "function": "fee_public",
                  "inputs": [
                    {
                      "type": "public",
                      "id": "7461282104433366449900697799474885740026008139066330886969703322575758055217field",
                      "value": "5000000u64"
                    },
                    {
                      "type": "public",
                      "id": "3934803756888630496501816151050540292915193353802102899830239800849008699942field",
                      "value": "1000000u64"
                    },
                    {
                      "type": "public",
                      "id": "2704568184889906689601787863190035940222705911013024881738685099334624579190field",
                      "value": "5143289311869305992819731554276892145943690667501397210953951405716358737411field"
                    }
                  ],
                  "outputs": [
                    {
                      "type": "future",
                      "id": "5847895945256167231966395890184269085803708731475986132553294535276357508888field",
                      "value": "{\n  program_id: credits.aleo,\n  function_name: fee_public,\n  arguments: [\n    aleo1yr9n35r0h6gazjfhajvy73u87f6nhc24dvhwel67lykrapf8fygsqv62ns,\n    6000000u64\n  ]\n}"
                    }
                  ],
                  "tpk": "3662889093089943591024614326755097771820280618862272357191320327772365699148group",
                  "tcm": "28343811668558083092323437539573502422062762506109415690591179678071251191field"
                }
              ]
            }
        "#;

        let transaction =
            commands::deploy_for_authorize(program, None, owner_str, fee_authorization_str, query);
        println!("transfer transaction: {}", transaction.unwrap());
    }
}
